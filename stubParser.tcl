namespace eval rteaStub {
    variable crateName "UNKNOWN"
    array set interfaces {}
    variable currIface "DEFAULT"
    array set hooks {}
    array set stubs {}

    variable srcDir .
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
        # puts $func
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

    # puts $currIface

    foreach {var item} $iface {
        set platform [lindex $item 0]
        set arg item[lindex $item 1]
        # puts "var:$var platform:$platform arg:$arg"
    }
    # puts $iface
    # return
    set interfaces($currIface) $iface
}

proc rteaStub::export {args} {}

proc rteaStub::init {} {
    global argv argv0
    # rteaStub::interface "DEFAULT"

    foreach file $argv {source -encoding utf-8 $file}
}

proc rteaStub::finalize {} {
    variable interfaces

    set max 0
    set genDefs [lindex [array get interfaces "tcl"] 1]

    dict set defaults 0 0

    # Output the vtable for `interpreter.rs`
    for  {set i 0} {$i < 700} {incr i} {
        if {[dict exists $genDefs $i]} {
            set decl [dict get $genDefs $i]
            if {[string match generic* [lindex $decl]]} {
                set info [lindex $decl 1]
                if {[lindex $info 0] == "TCL_NORTETURN"} {
                    set retString " -> !"
                } elseif {[lindex $info 1] == "UNUSED"} {
                    set retString ""
                } else {
                    set retString [lindex $info 1]
                    set retString " -> $retString"
                }
                set name [lindex $info 2]
                set params [lindex $info 3]

                if {$name == "TclUnusedStubEntry"} break

                set param_string ""
                foreach param $params {
                    if {"$param_string" != ""} {set param_string "$param_string, "}
                    set param_string "$param_string[lindex $param 0]"
                }

                puts "    $name: extern \"C\" fn($param_string)$retString, // $i"
            } else {
                puts "     _untranslated_$i: *const c_void, // $i"
            }
        } else {
            puts "     _deprecated_$i: *const c_void, // $i"
        }
    }

    # Output the internal function pointers for 
}

rteaStub::init
rteaStub::finalize
