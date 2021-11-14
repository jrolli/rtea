use std::ffi::c_void;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

use crate::Object;

#[repr(C)]
pub struct Interpreter {
    _legacy_result: *const c_void,
    _legace_free_proc: *const c_void,
    _error_line: isize,
    stubs: *const Stubs,
}

type CmdProc = fn(interp: &Interpreter, args: Vec<&str>) -> Result<TclStatus, String>;

const TCL_STUB_MAGIC: u32 = 0xFCA3BACF; // TCL 8.x extension

#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum TclStatus {
    Ok = 0,
    Error = 1,
    Return = 2,
    Break = 3,
    Continue = 4,
}

const _TCL_STATIC: *const c_void = 0 as *const c_void;
const _TCL_VOLATILE: *const c_void = 1 as *const c_void;
const TCL_DYNAMIC: *const c_void = 3 as *const c_void;

#[repr(C)]
struct Stubs {
    magic: u32,
    hooks: *const c_void,
    pkg_provide_ex:
        extern "C" fn(*const Interpreter, *const c_char, *const c_char, *const c_void) -> TclStatus,
    _untranslated_function1: *const c_void,  // 1
    _untranslated_function2: *const c_void,  // 2
    _untranslated_function3: *const c_void,  // 3
    _untranslated_function4: *const c_void,  // 4
    _untranslated_function5: *const c_void,  // 5
    _untranslated_function6: *const c_void,  // 6
    _untranslated_function7: *const c_void,  // 7
    _untranslated_function8: *const c_void,  // 8
    _untranslated_function9: *const c_void,  // 9
    _untranslated_function10: *const c_void, // 10
    _untranslated_function11: *const c_void, // 11
    _untranslated_function12: *const c_void, // 12
    _untranslated_function13: *const c_void, // 13
    _untranslated_function14: *const c_void, // 14
    _untranslated_function15: *const c_void, // 15
    _untranslated_function16: *const c_void, // 16
    _untranslated_function17: *const c_void, // 17
    _untranslated_function18: *const c_void, // 18
    _untranslated_function19: *const c_void, // 19
    _untranslated_function20: *const c_void, // 20
    _untranslated_function21: *const c_void, // 21
    _untranslated_function22: *const c_void, // 22
    _untranslated_function23: *const c_void, // 23
    _untranslated_function24: *const c_void, // 24
    _untranslated_function25: *const c_void, // 25
    _untranslated_function26: *const c_void, // 26
    _untranslated_function27: *const c_void, // 27
    _untranslated_function28: *const c_void, // 28
    _untranslated_function29: *const c_void, // 29
    _untranslated_function30: *const c_void, // 30
    _untranslated_function31: *const c_void, // 31
    _untranslated_function32: *const c_void, // 32
    _untranslated_function33: *const c_void, // 33
    _untranslated_function34: *const c_void, // 34
    _untranslated_function35: *const c_void, // 35
    _untranslated_function36: *const c_void, // 36
    _untranslated_function37: *const c_void, // 37
    _untranslated_function38: *const c_void, // 38
    _untranslated_function39: *const c_void, // 39
    _untranslated_function40: *const c_void, // 40
    _untranslated_function41: *const c_void, // 41
    _untranslated_function42: *const c_void, // 42
    _untranslated_function43: *const c_void, // 43
    _untranslated_function44: *const c_void, // 44
    _untranslated_function45: *const c_void, // 45
    _untranslated_function46: *const c_void, // 46
    _untranslated_function47: *const c_void, // 47
    _untranslated_function48: *const c_void, // 48
    _untranslated_function49: *const c_void, // 49
    _untranslated_function50: *const c_void, // 50
    _untranslated_function51: *const c_void, // 51
    _untranslated_function52: *const c_void, // 52
    _untranslated_function53: *const c_void, // 53
    _untranslated_function54: *const c_void, // 54
    _untranslated_function55: *const c_void, // 55
    _untranslated_function56: *const c_void, // 56
    _untranslated_function57: *const c_void, // 57
    _untranslated_function58: *const c_void, // 58
    _untranslated_function59: *const c_void, // 59
    _untranslated_function60: *const c_void, // 60
    _untranslated_function61: *const c_void, // 61
    _untranslated_function62: *const c_void, // 62
    _untranslated_function63: *const c_void, // 63
    _untranslated_function64: *const c_void, // 64
    _untranslated_function65: *const c_void, // 65
    _untranslated_function66: *const c_void, // 66
    _untranslated_function67: *const c_void, // 67
    _untranslated_function68: *const c_void, // 68
    _untranslated_function69: *const c_void, // 69
    _untranslated_function70: *const c_void, // 70
    _untranslated_function71: *const c_void, // 71
    _untranslated_function72: *const c_void, // 72
    _untranslated_function73: *const c_void, // 73
    _untranslated_function74: *const c_void, // 74
    _untranslated_function75: *const c_void, // 75
    _untranslated_function76: *const c_void, // 76
    _untranslated_function77: *const c_void, // 77
    _untranslated_function78: *const c_void, // 78
    _untranslated_function79: *const c_void, // 79
    _untranslated_function80: *const c_void, // 80
    _untranslated_function81: *const c_void, // 81
    _untranslated_function82: *const c_void, // 82
    _untranslated_function83: *const c_void, // 83
    _untranslated_function84: *const c_void, // 84
    _untranslated_function85: *const c_void, // 85
    _untranslated_function86: *const c_void, // 86
    _untranslated_function87: *const c_void, // 87
    _untranslated_function88: *const c_void, // 88
    _untranslated_function89: *const c_void, // 89
    _untranslated_function90: *const c_void, // 90
    create_command: extern "C" fn(
        *const Interpreter,
        *const c_char,
        *const c_void,
        *const c_void,
        *const c_void,
    ) -> *const c_void, // 91
    _untranslated_function92: *const c_void, // 92
    _untranslated_function93: *const c_void, // 93
    _untranslated_function94: *const c_void, // 94
    _untranslated_function95: *const c_void, // 95
    _untranslated_function96: *const c_void, // 96
    _untranslated_function97: *const c_void, // 97
    _untranslated_function98: *const c_void, // 98
    _untranslated_function99: *const c_void, // 99
    _untranslated_function100: *const c_void, // 100
    _untranslated_function101: *const c_void, // 101
    _untranslated_function102: *const c_void, // 102
    delete_command: extern "C" fn(*const Interpreter, *const c_char) -> isize, // 103
    _untranslated_function104: *const c_void, // 104
    _untranslated_function105: *const c_void, // 105
    _untranslated_function106: *const c_void, // 106
    _untranslated_function107: *const c_void, // 107
    _untranslated_function108: *const c_void, // 108
    _untranslated_function109: *const c_void, // 109
    _untranslated_function110: *const c_void, // 110
    _untranslated_function111: *const c_void, // 111
    _untranslated_function112: *const c_void, // 112
    _untranslated_function113: *const c_void, // 113
    _untranslated_function114: *const c_void, // 114
    _untranslated_function115: *const c_void, // 115
    _untranslated_function116: *const c_void, // 116
    _untranslated_function117: *const c_void, // 117
    _untranslated_function118: *const c_void, // 118
    _untranslated_function119: *const c_void, // 119
    _untranslated_function120: *const c_void, // 120
    _untranslated_function121: *const c_void, // 121
    _untranslated_function122: *const c_void, // 122
    _untranslated_function123: *const c_void, // 123
    _untranslated_function124: *const c_void, // 124
    _untranslated_function125: *const c_void, // 125
    _untranslated_function126: *const c_void, // 126
    _untranslated_function127: *const c_void, // 127
    _untranslated_function128: *const c_void, // 128
    _untranslated_function129: *const c_void, // 129
    _untranslated_function130: *const c_void, // 130
    _untranslated_function131: *const c_void, // 131
    _untranslated_function132: *const c_void, // 132
    _untranslated_function133: *const c_void, // 133
    _untranslated_function134: *const c_void, // 134
    _untranslated_function135: *const c_void, // 135
    _untranslated_function136: *const c_void, // 136
    _untranslated_function137: *const c_void, // 137
    _untranslated_function138: *const c_void, // 138
    _untranslated_function139: *const c_void, // 139
    _untranslated_function140: *const c_void, // 140
    _untranslated_function141: *const c_void, // 141
    _untranslated_function142: *const c_void, // 142
    _untranslated_function143: *const c_void, // 143
    _untranslated_function144: *const c_void, // 144
    _untranslated_function145: *const c_void, // 145
    _untranslated_function146: *const c_void, // 146
    _untranslated_function147: *const c_void, // 147
    _untranslated_function148: *const c_void, // 148
    _untranslated_function149: *const c_void, // 149
    _untranslated_function150: *const c_void, // 150
    _untranslated_function151: *const c_void, // 151
    _untranslated_function152: *const c_void, // 152
    _untranslated_function153: *const c_void, // 153
    _untranslated_function154: *const c_void, // 154
    _untranslated_function155: *const c_void, // 155
    _untranslated_function156: *const c_void, // 156
    _untranslated_function157: *const c_void, // 157
    _untranslated_function158: *const c_void, // 158
    _untranslated_function159: *const c_void, // 159
    _untranslated_function160: *const c_void, // 160
    _untranslated_function161: *const c_void, // 161
    _untranslated_function162: *const c_void, // 162
    _untranslated_function163: *const c_void, // 163
    _untranslated_function164: *const c_void, // 164
    _untranslated_function165: *const c_void, // 165
    get_obj_result: extern "C" fn(*const Interpreter) -> *const Object, // 166
    _untranslated_function167: *const c_void, // 167
    _untranslated_function168: *const c_void, // 168
    _untranslated_function169: *const c_void, // 169
    _untranslated_function170: *const c_void, // 170
    _untranslated_function171: *const c_void, // 171
    _untranslated_function172: *const c_void, // 172
    _untranslated_function173: *const c_void, // 173
    _untranslated_function174: *const c_void, // 174
    _untranslated_function175: *const c_void, // 175
    _untranslated_function176: *const c_void, // 176
    _untranslated_function177: *const c_void, // 177
    _untranslated_function178: *const c_void, // 178
    _untranslated_function179: *const c_void, // 179
    _untranslated_function180: *const c_void, // 180
    _untranslated_function181: *const c_void, // 181
    _untranslated_function182: *const c_void, // 182
    _untranslated_function183: *const c_void, // 183
    _untranslated_function184: *const c_void, // 184
    _untranslated_function185: *const c_void, // 185
    _untranslated_function186: *const c_void, // 186
    _untranslated_function187: *const c_void, // 187
    _untranslated_function188: *const c_void, // 188
    _untranslated_function189: *const c_void, // 189
    _untranslated_function190: *const c_void, // 190
    _untranslated_function191: *const c_void, // 191
    _untranslated_function192: *const c_void, // 192
    _untranslated_function193: *const c_void, // 193
    _untranslated_function194: *const c_void, // 194
    _untranslated_function195: *const c_void, // 195
    _untranslated_function196: *const c_void, // 196
    _untranslated_function197: *const c_void, // 197
    _untranslated_function198: *const c_void, // 198
    _untranslated_function199: *const c_void, // 199
    _untranslated_function200: *const c_void, // 200
    _untranslated_function201: *const c_void, // 201
    _untranslated_function202: *const c_void, // 202
    _untranslated_function203: *const c_void, // 203
    _untranslated_function204: *const c_void, // 204
    _untranslated_function205: *const c_void, // 205
    _untranslated_function206: *const c_void, // 206
    _untranslated_function207: *const c_void, // 207
    _untranslated_function208: *const c_void, // 208
    _untranslated_function209: *const c_void, // 209
    _untranslated_function210: *const c_void, // 210
    _untranslated_function211: *const c_void, // 211
    _untranslated_function212: *const c_void, // 212
    _untranslated_function213: *const c_void, // 213
    _untranslated_function214: *const c_void, // 214
    _untranslated_function215: *const c_void, // 215
    _untranslated_function216: *const c_void, // 216
    _untranslated_function217: *const c_void, // 217
    _untranslated_function218: *const c_void, // 218
    _untranslated_function219: *const c_void, // 219
    _untranslated_function220: *const c_void, // 220
    _untranslated_function221: *const c_void, // 221
    _untranslated_function222: *const c_void, // 222
    _untranslated_function223: *const c_void, // 223
    _untranslated_function224: *const c_void, // 224
    _untranslated_function225: *const c_void, // 225
    _untranslated_function226: *const c_void, // 226
    _untranslated_function227: *const c_void, // 227
    _untranslated_function228: *const c_void, // 228
    _untranslated_function229: *const c_void, // 229
    _untranslated_function230: *const c_void, // 230
    _untranslated_function231: *const c_void, // 231
    set_result: extern "C" fn(*const Interpreter, *const c_char, *const c_void), // 232
    _untranslated_function233: *const c_void, // 233
    _untranslated_function234: *const c_void, // 234
    _untranslated_function235: *const c_void, // 235
    _untranslated_function236: *const c_void, // 236
    _untranslated_function237: *const c_void, // 237
    _untranslated_function238: *const c_void, // 238
    _untranslated_function239: *const c_void, // 239
    _untranslated_function240: *const c_void, // 240
    _untranslated_function241: *const c_void, // 241
    _untranslated_function242: *const c_void, // 242
    _untranslated_function243: *const c_void, // 243
    _untranslated_function244: *const c_void, // 244
    _untranslated_function245: *const c_void, // 245
    _untranslated_function246: *const c_void, // 246
    _untranslated_function247: *const c_void, // 247
    _untranslated_function248: *const c_void, // 248
    _untranslated_function249: *const c_void, // 249
    _untranslated_function250: *const c_void, // 250
    _untranslated_function251: *const c_void, // 251
    _untranslated_function252: *const c_void, // 252
    _untranslated_function253: *const c_void, // 253
    _untranslated_function254: *const c_void, // 254
    _untranslated_function255: *const c_void, // 255
    _untranslated_function256: *const c_void, // 256
    _untranslated_function257: *const c_void, // 257
    _untranslated_function258: *const c_void, // 258
    _untranslated_function259: *const c_void, // 259
    _untranslated_function260: *const c_void, // 260
    _untranslated_function261: *const c_void, // 261
    _untranslated_function262: *const c_void, // 262
    _untranslated_function263: *const c_void, // 263
    _untranslated_function264: *const c_void, // 264
    _untranslated_function265: *const c_void, // 265
    _untranslated_function266: *const c_void, // 266
    _untranslated_function267: *const c_void, // 267
    _untranslated_function268: *const c_void, // 268
    _untranslated_function269: *const c_void, // 269
    _untranslated_function270: *const c_void, // 270
    _untranslated_function271: *const c_void, // 271
    _untranslated_function272: *const c_void, // 272
    _untranslated_function273: *const c_void, // 273
    _untranslated_function274: *const c_void, // 274
    _untranslated_function275: *const c_void, // 275
    _untranslated_function276: *const c_void, // 276
    _untranslated_function277: *const c_void, // 277
    _untranslated_function278: *const c_void, // 278
    _untranslated_function279: *const c_void, // 279
    _untranslated_function280: *const c_void, // 280
    _untranslated_function281: *const c_void, // 281
    _untranslated_function282: *const c_void, // 282
    _untranslated_function283: *const c_void, // 283
    _untranslated_function284: *const c_void, // 284
    _untranslated_function285: *const c_void, // 285
    _untranslated_function286: *const c_void, // 286
    _untranslated_function287: *const c_void, // 287
    _untranslated_function288: *const c_void, // 288
    _untranslated_function289: *const c_void, // 289
    _untranslated_function290: *const c_void, // 290
    eval_ex: extern "C" fn(*const Interpreter, *const c_char, usize, i32) -> TclStatus, // 291
    _untranslated_function292: *const c_void, // 292
    _untranslated_function293: *const c_void, // 293
    _untranslated_function294: *const c_void, // 294
    _untranslated_function295: *const c_void, // 295
    _untranslated_function296: *const c_void, // 296
    _untranslated_function297: *const c_void, // 297
    _untranslated_function298: *const c_void, // 298
    _untranslated_function299: *const c_void, // 299
    _untranslated_function300: *const c_void, // 300
    _untranslated_function301: *const c_void, // 301
    _untranslated_function302: *const c_void, // 302
    _untranslated_function303: *const c_void, // 303
    _untranslated_function304: *const c_void, // 304
    _untranslated_function305: *const c_void, // 305
    _untranslated_function306: *const c_void, // 306
    _untranslated_function307: *const c_void, // 307
    _untranslated_function308: *const c_void, // 308
    _untranslated_function309: *const c_void, // 309
    _untranslated_function310: *const c_void, // 310
    _untranslated_function311: *const c_void, // 311
    _untranslated_function312: *const c_void, // 312
    _untranslated_function313: *const c_void, // 313
    _untranslated_function314: *const c_void, // 314
    _untranslated_function315: *const c_void, // 315
    _untranslated_function316: *const c_void, // 316
    _untranslated_function317: *const c_void, // 317
    _untranslated_function318: *const c_void, // 318
    _untranslated_function319: *const c_void, // 319
    _untranslated_function320: *const c_void, // 320
    _untranslated_function321: *const c_void, // 321
    _untranslated_function322: *const c_void, // 322
    _untranslated_function323: *const c_void, // 323
    _untranslated_function324: *const c_void, // 324
    _untranslated_function325: *const c_void, // 325
    _untranslated_function326: *const c_void, // 326
    _untranslated_function327: *const c_void, // 327
    _untranslated_function328: *const c_void, // 328
    _untranslated_function329: *const c_void, // 329
    _untranslated_function330: *const c_void, // 330
    _untranslated_function331: *const c_void, // 331
    _untranslated_function332: *const c_void, // 332
    _untranslated_function333: *const c_void, // 333
    _untranslated_function334: *const c_void, // 334
    _untranslated_function335: *const c_void, // 335
    _untranslated_function336: *const c_void, // 336
    _untranslated_function337: *const c_void, // 337
    _untranslated_function338: *const c_void, // 338
    _untranslated_function339: *const c_void, // 339
    get_string: extern "C" fn(*const Object) -> *const c_char, // 340
    _untranslated_function341: *const c_void, // 341
    _untranslated_function342: *const c_void, // 342
    _untranslated_function343: *const c_void, // 343
    _untranslated_function344: *const c_void, // 344
    _untranslated_function345: *const c_void, // 345
    _untranslated_function346: *const c_void, // 346
    _untranslated_function347: *const c_void, // 347
    _untranslated_function348: *const c_void, // 348
    _untranslated_function349: *const c_void, // 349
    _untranslated_function350: *const c_void, // 350
    _untranslated_function351: *const c_void, // 351
    _untranslated_function352: *const c_void, // 352
    _untranslated_function353: *const c_void, // 353
    _untranslated_function354: *const c_void, // 354
    _untranslated_function355: *const c_void, // 355
    _untranslated_function356: *const c_void, // 356
    _untranslated_function357: *const c_void, // 357
    _untranslated_function358: *const c_void, // 358
    _untranslated_function359: *const c_void, // 359
    _untranslated_function360: *const c_void, // 360
    _untranslated_function361: *const c_void, // 361
    _untranslated_function362: *const c_void, // 362
    _untranslated_function363: *const c_void, // 363
    _untranslated_function364: *const c_void, // 364
    _untranslated_function365: *const c_void, // 365
    _untranslated_function366: *const c_void, // 366
    _untranslated_function367: *const c_void, // 367
    _untranslated_function368: *const c_void, // 368
    _untranslated_function369: *const c_void, // 369
    _untranslated_function370: *const c_void, // 370
    _untranslated_function371: *const c_void, // 371
    _untranslated_function372: *const c_void, // 372
    _untranslated_function373: *const c_void, // 373
    _untranslated_function374: *const c_void, // 374
    _untranslated_function375: *const c_void, // 375
    _untranslated_function376: *const c_void, // 376
    _untranslated_function377: *const c_void, // 377
    _untranslated_function378: *const c_void, // 378
    _untranslated_function379: *const c_void, // 379
    _untranslated_function380: *const c_void, // 380
    _untranslated_function381: *const c_void, // 381
    _untranslated_function382: *const c_void, // 382
    _untranslated_function383: *const c_void, // 383
    _untranslated_function384: *const c_void, // 384
    _untranslated_function385: *const c_void, // 385
    _untranslated_function386: *const c_void, // 386
    _untranslated_function387: *const c_void, // 387
    _untranslated_function388: *const c_void, // 388
    _untranslated_function389: *const c_void, // 389
    _untranslated_function390: *const c_void, // 390
    _untranslated_function391: *const c_void, // 391
    _untranslated_function392: *const c_void, // 392
    _untranslated_function393: *const c_void, // 393
    _untranslated_function394: *const c_void, // 394
    _untranslated_function395: *const c_void, // 395
    _untranslated_function396: *const c_void, // 396
    _untranslated_function397: *const c_void, // 397
    _untranslated_function398: *const c_void, // 398
    _untranslated_function399: *const c_void, // 399
    _untranslated_function400: *const c_void, // 400
    _untranslated_function401: *const c_void, // 401
    _untranslated_function402: *const c_void, // 402
    _untranslated_function403: *const c_void, // 403
    _untranslated_function404: *const c_void, // 404
    _untranslated_function405: *const c_void, // 405
    _untranslated_function406: *const c_void, // 406
    _untranslated_function407: *const c_void, // 407
    _untranslated_function408: *const c_void, // 408
    _untranslated_function409: *const c_void, // 409
    _untranslated_function410: *const c_void, // 410
    _untranslated_function411: *const c_void, // 411
    _untranslated_function412: *const c_void, // 412
    _untranslated_function413: *const c_void, // 413
    _untranslated_function414: *const c_void, // 414
    _untranslated_function415: *const c_void, // 415
    _untranslated_function416: *const c_void, // 416
    _untranslated_function417: *const c_void, // 417
    _untranslated_function418: *const c_void, // 418
    _untranslated_function419: *const c_void, // 419
    _untranslated_function420: *const c_void, // 420
    _untranslated_function421: *const c_void, // 421
    _untranslated_function422: *const c_void, // 422
    _untranslated_function423: *const c_void, // 423
    _untranslated_function424: *const c_void, // 424
    _untranslated_function425: *const c_void, // 425
    _untranslated_function426: *const c_void, // 426
    _untranslated_function427: *const c_void, // 427
    attempt_alloc: extern "C" fn(usize) -> *mut u8, // 428
    _untranslated_function429: *const c_void, // 429
    _untranslated_function430: *const c_void, // 430
    _untranslated_function431: *const c_void, // 431
    _untranslated_function432: *const c_void, // 432
    _untranslated_function433: *const c_void, // 433
    _untranslated_function434: *const c_void, // 434
    _untranslated_function435: *const c_void, // 435
    _untranslated_function436: *const c_void, // 436
    _untranslated_function437: *const c_void, // 437
    _untranslated_function438: *const c_void, // 438
    _untranslated_function439: *const c_void, // 439
    _untranslated_function440: *const c_void, // 440
    _untranslated_function441: *const c_void, // 441
    _untranslated_function442: *const c_void, // 442
    _untranslated_function443: *const c_void, // 443
    _untranslated_function444: *const c_void, // 444
    _untranslated_function445: *const c_void, // 445
    _untranslated_function446: *const c_void, // 446
    _untranslated_function447: *const c_void, // 447
    _untranslated_function448: *const c_void, // 448
    _untranslated_function449: *const c_void, // 449
    _untranslated_function450: *const c_void, // 450
    _untranslated_function451: *const c_void, // 451
    _untranslated_function452: *const c_void, // 452
    _untranslated_function453: *const c_void, // 453
    _untranslated_function454: *const c_void, // 454
    _untranslated_function455: *const c_void, // 455
    _untranslated_function456: *const c_void, // 456
    _untranslated_function457: *const c_void, // 457
    _untranslated_function458: *const c_void, // 458
    _untranslated_function459: *const c_void, // 459
    _untranslated_function460: *const c_void, // 460
    _untranslated_function461: *const c_void, // 461
    _untranslated_function462: *const c_void, // 462
    _untranslated_function463: *const c_void, // 463
    _untranslated_function464: *const c_void, // 464
    _untranslated_function465: *const c_void, // 465
    _untranslated_function466: *const c_void, // 466
    _untranslated_function467: *const c_void, // 467
    _untranslated_function468: *const c_void, // 468
    _untranslated_function469: *const c_void, // 469
    _untranslated_function470: *const c_void, // 470
    _untranslated_function471: *const c_void, // 471
    _untranslated_function472: *const c_void, // 472
    _untranslated_function473: *const c_void, // 473
    _untranslated_function474: *const c_void, // 474
    _untranslated_function475: *const c_void, // 475
    _untranslated_function476: *const c_void, // 476
    _untranslated_function477: *const c_void, // 477
    _untranslated_function478: *const c_void, // 478
    _untranslated_function479: *const c_void, // 479
    _untranslated_function480: *const c_void, // 480
    _untranslated_function481: *const c_void, // 481
    _untranslated_function482: *const c_void, // 482
    _untranslated_function483: *const c_void, // 483
    _untranslated_function484: *const c_void, // 484
    _untranslated_function485: *const c_void, // 485
    _untranslated_function486: *const c_void, // 486
    _untranslated_function487: *const c_void, // 487
    _untranslated_function488: *const c_void, // 488
    _untranslated_function489: *const c_void, // 489
    _untranslated_function490: *const c_void, // 490
    _untranslated_function491: *const c_void, // 491
    _untranslated_function492: *const c_void, // 492
    _untranslated_function493: *const c_void, // 493
    _untranslated_function494: *const c_void, // 494
    _untranslated_function495: *const c_void, // 495
    _untranslated_function496: *const c_void, // 496
    _untranslated_function497: *const c_void, // 497
    _untranslated_function498: *const c_void, // 498
    _untranslated_function499: *const c_void, // 499
    _untranslated_function500: *const c_void, // 500
    _untranslated_function501: *const c_void, // 501
    _untranslated_function502: *const c_void, // 502
    _untranslated_function503: *const c_void, // 503
    _untranslated_function504: *const c_void, // 504
    _untranslated_function505: *const c_void, // 505
    _untranslated_function506: *const c_void, // 506
    _untranslated_function507: *const c_void, // 507
    _untranslated_function508: *const c_void, // 508
    _untranslated_function509: *const c_void, // 509
    _untranslated_function510: *const c_void, // 510
    _untranslated_function511: *const c_void, // 511
    _untranslated_function512: *const c_void, // 512
    _untranslated_function513: *const c_void, // 513
    _untranslated_function514: *const c_void, // 514
    _untranslated_function515: *const c_void, // 515
    _untranslated_function516: *const c_void, // 516
    _untranslated_function517: *const c_void, // 517
    _untranslated_function518: *const c_void, // 518
    _untranslated_function519: *const c_void, // 519
    _untranslated_function520: *const c_void, // 520
    _untranslated_function521: *const c_void, // 521
    _untranslated_function522: *const c_void, // 522
    _untranslated_function523: *const c_void, // 523
    _untranslated_function524: *const c_void, // 524
    _untranslated_function525: *const c_void, // 525
    _untranslated_function526: *const c_void, // 526
    _untranslated_function527: *const c_void, // 527
    _untranslated_function528: *const c_void, // 528
    _untranslated_function529: *const c_void, // 529
    _untranslated_function530: *const c_void, // 530
    _untranslated_function531: *const c_void, // 531
    _untranslated_function532: *const c_void, // 532
    _untranslated_function533: *const c_void, // 533
    _untranslated_function534: *const c_void, // 534
    _untranslated_function535: *const c_void, // 535
    _untranslated_function536: *const c_void, // 536
    _untranslated_function537: *const c_void, // 537
    _untranslated_function538: *const c_void, // 538
    _untranslated_function539: *const c_void, // 539
    _untranslated_function540: *const c_void, // 540
    _untranslated_function541: *const c_void, // 541
    _untranslated_function542: *const c_void, // 542
    _untranslated_function543: *const c_void, // 543
    _untranslated_function544: *const c_void, // 544
    _untranslated_function545: *const c_void, // 545
    _untranslated_function546: *const c_void, // 546
    _untranslated_function547: *const c_void, // 547
    _untranslated_function548: *const c_void, // 548
    _untranslated_function549: *const c_void, // 549
    _untranslated_function550: *const c_void, // 550
    _untranslated_function551: *const c_void, // 551
    _untranslated_function552: *const c_void, // 552
    _untranslated_function553: *const c_void, // 553
    _untranslated_function554: *const c_void, // 554
    _untranslated_function555: *const c_void, // 555
    _untranslated_function556: *const c_void, // 556
    _untranslated_function557: *const c_void, // 557
    _untranslated_function558: *const c_void, // 558
    _untranslated_function559: *const c_void, // 559
    _untranslated_function560: *const c_void, // 560
    _untranslated_function561: *const c_void, // 561
    _untranslated_function562: *const c_void, // 562
    _untranslated_function563: *const c_void, // 563
    _untranslated_function564: *const c_void, // 564
    _untranslated_function565: *const c_void, // 565
    _untranslated_function566: *const c_void, // 566
    _untranslated_function567: *const c_void, // 567
    _untranslated_function568: *const c_void, // 568
    _untranslated_function569: *const c_void, // 569
    _untranslated_function570: *const c_void, // 570
    _untranslated_function571: *const c_void, // 571
    _untranslated_function572: *const c_void, // 572
    _untranslated_function573: *const c_void, // 573
    _untranslated_function574: *const c_void, // 574
    _untranslated_function575: *const c_void, // 575
    _untranslated_function576: *const c_void, // 576
    _untranslated_function577: *const c_void, // 577
    _untranslated_function578: *const c_void, // 578
    _untranslated_function579: *const c_void, // 579
    _untranslated_function580: *const c_void, // 580
    _untranslated_function581: *const c_void, // 581
    _untranslated_function582: *const c_void, // 582
    _untranslated_function583: *const c_void, // 583
    _untranslated_function584: *const c_void, // 584
    _untranslated_function585: *const c_void, // 585
    _untranslated_function586: *const c_void, // 586
    _untranslated_function587: *const c_void, // 587
    _untranslated_function588: *const c_void, // 588
    _untranslated_function589: *const c_void, // 589
    _untranslated_function590: *const c_void, // 590
    _untranslated_function591: *const c_void, // 591
    _untranslated_function592: *const c_void, // 592
    _untranslated_function593: *const c_void, // 593
    _untranslated_function594: *const c_void, // 594
    _untranslated_function595: *const c_void, // 595
    _untranslated_function596: *const c_void, // 596
    _untranslated_function597: *const c_void, // 597
    _untranslated_function598: *const c_void, // 598
    _untranslated_function599: *const c_void, // 599
    _untranslated_function600: *const c_void, // 600
    _untranslated_function601: *const c_void, // 601
    _untranslated_function602: *const c_void, // 602
    _untranslated_function603: *const c_void, // 603
    _untranslated_function604: *const c_void, // 604
    _untranslated_function605: *const c_void, // 605
    _untranslated_function606: *const c_void, // 606
    _untranslated_function607: *const c_void, // 607
    _untranslated_function608: *const c_void, // 608
    _untranslated_function609: *const c_void, // 609
    _untranslated_function610: *const c_void, // 610
    _untranslated_function611: *const c_void, // 611
    _untranslated_function612: *const c_void, // 612
    _untranslated_function613: *const c_void, // 613
    _untranslated_function614: *const c_void, // 614
    _untranslated_function615: *const c_void, // 615
    _untranslated_function616: *const c_void, // 616
    _untranslated_function617: *const c_void, // 617
    _untranslated_function618: *const c_void, // 618
    _untranslated_function619: *const c_void, // 619
    _untranslated_function620: *const c_void, // 620
    _untranslated_function621: *const c_void, // 621
    _untranslated_function622: *const c_void, // 622
    _untranslated_function623: *const c_void, // 623
    _untranslated_function624: *const c_void, // 624
    _untranslated_function625: *const c_void, // 625
    _untranslated_function626: *const c_void, // 626
    _untranslated_function627: *const c_void, // 627
    _untranslated_function628: *const c_void, // 628
    _untranslated_function629: *const c_void, // 629
    _untranslated_function630: *const c_void, // 630
    _untranslated_function631: *const c_void, // 631
    _untranslated_function632: *const c_void, // 632
    _untranslated_function633: *const c_void, // 633
    _untranslated_function634: *const c_void, // 634
    _untranslated_function635: *const c_void, // 635
    _untranslated_function636: *const c_void, // 636
    _untranslated_function637: *const c_void, // 637
    _untranslated_function638: *const c_void, // 638
    _untranslated_function639: *const c_void, // 639
    _untranslated_function640: *const c_void, // 640
    _untranslated_function641: *const c_void, // 641
    _untranslated_function642: *const c_void, // 642
    _untranslated_function643: *const c_void, // 643
    _untranslated_function644: *const c_void, // 644
    _untranslated_function645: *const c_void, // 645
    _untranslated_function646: *const c_void, // 646
    _untranslated_function647: *const c_void, // 647
    _untranslated_function648: *const c_void, // 648
    _untranslated_function649: *const c_void, // 649
}

#[derive(Debug)]
pub enum Error {
    NullInterpreter,
    NullStubs,
    InvalidStubs,
    TclError(String),
}

impl<'a> Interpreter {
    pub fn from_raw(interpreter: *const Interpreter) -> Result<&'a Interpreter, Error> {
        if let Some(interpreter) = unsafe { interpreter.as_ref() } {
            if let Some(stubs) = unsafe { interpreter.stubs.as_ref() } {
                if stubs.magic == TCL_STUB_MAGIC {
                    Ok(interpreter)
                } else {
                    Err(Error::InvalidStubs)
                }
            } else {
                Err(Error::NullStubs)
            }
        } else {
            Err(Error::NullInterpreter)
        }
    }

    pub fn provide_package(&self, name: &str, version: &str) -> Result<TclStatus, String> {
        let name =
            CString::new(name).map_err(|_| "unexpected Nul in package version".to_string())?;
        let version =
            CString::new(version).map_err(|_| "unexpected Nul in package version".to_string())?;
        unsafe {
            (self
                .stubs
                .as_ref()
                .expect("stubs missing after initial check")
                .pkg_provide_ex)(
                self as *const Interpreter,
                name.as_ptr(),
                version.as_ptr(),
                std::ptr::null(),
            )
        };
        Ok(TclStatus::Ok)
    }

    pub fn create_command(&self, name: &str, proc: CmdProc) -> Result<TclStatus, String> {
        let name = CString::new(name).map_err(|_| "unexpected Nul in command name".to_string())?;

        // type TclCmdProc = extern "C" fn(
        //     data: *const c_void,
        //     interp: *const Interpreter,
        //     argc: usize,
        //     argv: *const *const i8,
        // ) -> TclStatus;
        fn wrapper_proc(
            f: CmdProc,
            i: *const Interpreter,
            argc: usize,
            argv: *const *const i8,
        ) -> TclStatus {
            let interp = Interpreter::from_raw(i).expect("TCL passed bad interpreter");
            let raw_args = unsafe { std::slice::from_raw_parts(argv, argc) };
            let mut args = Vec::with_capacity(raw_args.len());
            for arg in raw_args {
                args.push(
                    unsafe { std::ffi::CStr::from_ptr(*arg) }
                        .to_str()
                        .expect("invalid args from TCL"),
                );
            }

            f(interp, args).unwrap_or_else(|s| {
                interp.set_result(&s);
                TclStatus::Error
            })
        }

        unsafe {
            (self
                .stubs
                .as_ref()
                .expect("stubs missing after initial check")
                .create_command)(
                self as *const Interpreter,
                name.as_ptr(),
                wrapper_proc as *const c_void,
                proc as *const c_void,
                std::ptr::null() as *const c_void,
            )
        };

        Ok(TclStatus::Ok)
    }

    pub fn delete_command(&self, name: &str) -> Result<bool, String> {
        let name = CString::new(name).map_err(|_| "unexpected Nul in command name".to_string())?;

        let ret = unsafe {
            (self
                .stubs
                .as_ref()
                .expect("stubs missing after initial check")
                .delete_command)(self as *const Interpreter, name.as_ptr())
        };

        Ok(ret == 0)
    }

    pub fn get_obj_result(&self) -> &Object {
        unsafe {
            (self
                .stubs
                .as_ref()
                .expect("stubs missing after initial check")
                .get_obj_result)(self as *const Interpreter)
            .as_ref()
            .expect("TCL should guarantee this is not Null")
        }
    }

    pub fn eval(&self, script: &str) -> Result<TclStatus, String> {
        if script.len() > 1 << 31 {
            return Err(
                "TCL versions prior to 9.0 do not support scripts greater than 2 GiB".to_string(),
            );
        }
        let res = unsafe {
            (self
                .stubs
                .as_ref()
                .expect("stubs missing after initial check")
                .eval_ex)(
                self as *const Interpreter,
                script.as_ptr() as *const c_char,
                script.len(),
                0,
            )
        };
        if res == TclStatus::Error {
            Err(self.get_string(self.get_obj_result()))
        } else {
            Ok(res)
        }
    }

    pub fn set_result(&self, text: &str) {
        let tcl_str = self
            .alloc(text.len() + 1)
            .expect("propagating memory failure in TCL");

        tcl_str[..text.len()].copy_from_slice(text.as_bytes());

        if let Some(terminator) = tcl_str.last_mut() {
            *terminator = 0;
        }

        unsafe {
            (self
                .stubs
                .as_ref()
                .expect("stubs missing after initial check")
                .set_result)(
                self as *const Interpreter,
                tcl_str.as_ptr() as *const c_char,
                TCL_DYNAMIC,
            )
        }
    }

    pub fn get_string(&self, obj: &Object) -> String {
        let raw = unsafe {
            // Trusting TCL
            CStr::from_ptr((self
                .stubs
                .as_ref()
                .expect("stubs missing after initial check")
                .get_string)(obj))
        };
        raw.to_str().expect("Invalid UTF-8 from TCL").to_string()
    }

    pub fn alloc(&self, size: usize) -> Option<&mut [u8]> {
        if size >= 1 << 32 {
            return None;
        }
        let ptr = unsafe {
            // Trusting TCL to handle this correctly (check above can be removed for TCL 9.0)
            (self
                .stubs
                .as_ref()
                .expect("stubs missing after initial check")
                .attempt_alloc)(size)
        };

        if ptr.is_null() {
            None
        } else {
            unsafe {
                // We've checked that it is not null and therefore trust TCL
                Some(std::slice::from_raw_parts_mut(ptr, size))
            }
        }
    }
}

type CmdDataProc<T> =
    fn(interp: &Interpreter, data: &T, args: Vec<&str>) -> Result<TclStatus, String>;

pub struct StatefulCommand<T> {
    proc: CmdDataProc<T>,
    data: T,
}

impl<T> StatefulCommand<T> {
    pub fn new(proc: CmdDataProc<T>, data: T) -> StatefulCommand<T> {
        StatefulCommand::<T> {
            proc: proc,
            data: data,
        }
    }

    pub fn attach_command(self, interp: &Interpreter, name: &str) -> Result<TclStatus, String> {
        let state = Box::new(self);
        let name = CString::new(name).map_err(|_| "unexpected Nul in command name".to_string())?;

        fn wrapper_proc<T>(
            state: *const StatefulCommand<T>,
            i: *const Interpreter,
            argc: usize,
            argv: *const *const i8,
        ) -> TclStatus {
            let interp = Interpreter::from_raw(i).expect("TCL passed bad interpreter");
            let raw_args = unsafe { std::slice::from_raw_parts(argv, argc) };
            let mut args = Vec::with_capacity(raw_args.len());
            for arg in raw_args {
                args.push(
                    unsafe { std::ffi::CStr::from_ptr(*arg) }
                        .to_str()
                        .expect("invalid args from TCL"),
                );
            }

            let state = unsafe { state.as_ref() }.expect("data command corrupted!");

            (state.proc)(interp, &state.data, args).unwrap_or_else(|s| {
                interp.set_result(&s);
                TclStatus::Error
            })
        }

        fn free_state<T>(state: *mut StatefulCommand<T>) {
            // This relies on TCL to properly track the command state and
            // invoke this at the appropriate moment.  Retaking ownership
            // of the underlying pointer ensures the destructor gets called
            unsafe { Box::<StatefulCommand<T>>::from_raw(state) };
        }

        unsafe {
            (interp
                .stubs
                .as_ref()
                .expect("stubs missing after initial check")
                .create_command)(
                interp as *const Interpreter,
                name.as_ptr(),
                wrapper_proc::<T> as *const c_void,
                Box::<StatefulCommand<T>>::into_raw(state) as *const c_void,
                free_state::<T> as *const c_void,
            )
        };

        Ok(TclStatus::Ok)
    }
}
