# stubParser.tcl -- Regenerate the Tcl stubs vtable in src/interpreter.rs
#
# This script reads a Tcl stubs declaration file (tcl.decls) and rewrites the
# `Stubs` struct in src/interpreter.rs with an up-to-date vtable derived
# directly from the Tcl source tree.
#
# How it works:
#   1. The script sources the given .decls file, which calls back into the
#      procs defined here (library, interface, hooks, declare, etc.) -- the
#      same protocol used by Tcl's own genStubs.tcl tooling.
#   2. Each `declare` call is recorded in the `interfaces(tcl)` dict, keyed
#      by slot number.  Slots present in the dict become typed `extern "C" fn`
#      fields; absent slots become `_deprecated_N: *const c_void` placeholders.
#   3. C types are translated to Rust FFI types by `convertstub` (e.g.
#      `char *` -> `*mut c_char`, `Tcl_Interp *` -> `*const Interpreter`).
#   4. The generated struct replaces the region of src/interpreter.rs that is
#      delimited by the sentinel comments:
#          // BEGIN STUBS VTABLE
#          // END STUBS VTABLE
#      Everything outside those sentinels is left untouched.
#
# Usage:
#   tclsh stubParser.tcl /path/to/tcl/generic/tcl.decls
#
# Example (from the rtea project root, with the Tcl source tree alongside):
#   tclsh stubParser.tcl ../tcl/tcl/generic/tcl.decls

namespace eval rteaStub {
    variable crateName "UNKNOWN"
    array set interfaces {}
    variable currIface "DEFAULT"
    array set hooks {}
    array set stubs {}

    variable srcDir [file dirname [file normalize [info script]]]
}

proc rteaStub::library {name} {
    variable crateName "$name-sys"
}

proc rteaStub::interface {name} {
    variable currIface $name
    variable interfaces

    set interfaces($currIface) {}
}

proc rteaStub::scspec {args} {}

proc rteaStub::hooks {names} {
    variable currIface
    variable hooks

    set hooks($currIface) $names
}

proc rteaStub::convertstub {kind} {
    switch -glob $kind {
        {Tcl_Interp \*}        {set result {*const Interpreter}}
        {Tcl_Obj \*}           {set result {*mut   RawObject}}
        {const Tcl_ObjType \*} {set result {*const ObjectType}}
        {Tcl_Size}             {set result {usize}}
        {const char \*}        {set result {*const c_char}}
        {char \*}              {set result {*mut c_char}}
        {const*\*}             {set result {*const c_void}}
        {void}                 {set result {UNUSED}}
        {void \*}              {set result {*mut c_void}}
        {const void \*}        {set result {*const c_void}}
        {char}                 {set result {c_char}}
        {double}               {set result {c_double}}
        {float}                {set result {c_float}}
        {int}                  {set result {c_int}}
        {long}                 {set result {c_long}}
        {long long}            {set result {c_longlong}}
        {short}                {set result {c_short}}
        {unsigned char}        {set result {c_uchar}}
        {unsigned int}         {set result {c_uint}}
        {unsigned long}        {set result {c_ulong}}
        {unsigned long long}   {set result {c_ulonglong}}
        {unsigned short}       {set result {c_ushort}}
        {TCL_HASH_TYPE}        {set result {usize}}
        default                {set result {*mut c_void} }
    }
}

proc rteaStub::parsedecl {decl} {
    regexp {([a-zA-Z][a-zA-Z0-9_*\s]+)\s*([[:<:]][a-zA-Z0-9_]+)\(([^)]*)\)} $decl line meta name args
    if {![regexp {^([A-Z_]{2,})\s*([A-Za-z][a-z][a-zA-Z0-9_\s]+)} $meta line attr returntype]} {
        set attr {}
        set returntype $meta
    }
    set returntype [convertstub [string trim $returntype]]
    set args [split $args ","]
    if {$args == "void"} {
        set args {}
    } else {
        set args [lmap arg $args {
            regexp {(.+)[[:<:]](\w+)} $arg line kind varname
            list [convertstub [string trim $kind]] $varname
        }]
    }
    return [list $attr $returntype $name $args]
}

proc rteaStub::declare {args} {
    variable crateName
    variable interfaces
    variable currIface
    set iface [lindex [array get interfaces $currIface] 1]

    if {[llength $args] == 2} {
        set func [rteaStub::parsedecl [lindex $args 1]]
        dict set iface [lindex $args 0] [list generic $func]
    } elseif {[llength $args] == 3} {
        set func [rteaStub::parsedecl [lindex $args 2]]
        set platform [lindex $args 1]
        if {[string match "nostub *" $platform] || [string match "deprecated *" $platform]} {
            set $platform [lindex [lindex $args 1] 0]
        }
        dict set iface [lindex $args 0] [list $platform $func]
    } else {
        puts stderr "wrong # args: declare $args"
        return
    }

    foreach {var item} $iface {
        set platform [lindex $item 0]
        set arg item[lindex $item 1]
    }
    set interfaces($currIface) $iface
}

proc rteaStub::export {args} {}

proc rteaStub::init {} {
    global argv argv0

    foreach file $argv {source -encoding utf-8 $file}
}

proc rteaStub::generate_vtable {} {
    variable interfaces

    set genDefs [lindex [array get interfaces "tcl"] 1]
    set lines {}

    lappend lines "#\[repr(C)\]"
    lappend lines "#\[allow(non_snake_case)\]"
    lappend lines "#\[derive(Debug)\]"
    lappend lines "struct Stubs \{"
    lappend lines "    magic: u32,"
    lappend lines "    hooks: *const c_void,"

    for {set i 0} {$i < 1000} {incr i} {
        if {[dict exists $genDefs $i]} {
            set decl [dict get $genDefs $i]
            if {[string match generic* [lindex $decl 0]]} {
                set info [lindex $decl 1]

                if {[lindex $info 0] == "TCL_NORTETURN"} {
                    set retString " -> !"
                } elseif {[lindex $info 1] == "UNUSED"} {
                    set retString ""
                } else {
                    set retString " -> [lindex $info 1]"
                }

                set name [lindex $info 2]

                if {$name == "TclUnusedStubEntry"} break

                set params [lindex $info 3]
                set param_string ""
                foreach param $params {
                    if {$param_string ne ""} {append param_string ", "}
                    append param_string [lindex $param 0]
                }

                lappend lines "    $name: extern \"C\" fn($param_string)$retString, // $i"
            } else {
                lappend lines "     _untranslated_$i: *const c_void, // $i"
            }
        } else {
            lappend lines "     _deprecated_$i: *const c_void, // $i"
        }
    }

    lappend lines "\}"

    return [join $lines "\n"]
}

proc rteaStub::finalize {} {
    variable srcDir

    set targetFile [file join $srcDir src interpreter.rs]

    if {![file exists $targetFile]} {
        puts stderr "error: cannot find $targetFile"
        exit 1
    }

    # Read the existing file
    set fd [open $targetFile r]
    fconfigure $fd -encoding utf-8
    set content [read $fd]
    close $fd

    set lines [split $content "\n"]

    # Locate the sentinel lines
    set beginIdx -1
    set endIdx   -1
    for {set i 0} {$i < [llength $lines]} {incr i} {
        set line [lindex $lines $i]
        if {[string match "// BEGIN STUBS VTABLE*" $line]} {
            set beginIdx $i
        } elseif {$line eq "// END STUBS VTABLE"} {
            set endIdx $i
            break
        }
    }

    if {$beginIdx == -1} {
        puts stderr "error: could not find '// BEGIN STUBS VTABLE' sentinel in $targetFile"
        exit 1
    }
    if {$endIdx == -1} {
        puts stderr "error: could not find '// END STUBS VTABLE' sentinel in $targetFile"
        exit 1
    }
    if {$endIdx <= $beginIdx} {
        puts stderr "error: END sentinel appears before BEGIN sentinel in $targetFile"
        exit 1
    }

    # Build the replacement: keep the sentinels, replace everything between them
    set before  [lrange $lines 0 $beginIdx]
    set after   [lrange $lines $endIdx end]
    set vtable  [generate_vtable]

    set newLines [concat $before [split $vtable "\n"] $after]
    set newContent [join $newLines "\n"]

    # Write the updated file back
    set fd [open $targetFile w]
    fconfigure $fd -encoding utf-8
    puts -nonewline $fd $newContent
    close $fd

    puts "Updated Stubs vtable in $targetFile"
}

rteaStub::init
rteaStub::finalize
