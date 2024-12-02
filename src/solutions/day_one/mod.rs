pub mod part_one;
pub mod part_two;

pub const LEFT: [i32; 1000] = [
  16435,
  29877,
  75256,
  25417,
  32479,
  93953,
  21297,
  41677,
  42091,
  74831,
  53533,
  83870,
  78929,
  85446,
  31544,
  44640,
  34692,
  58962,
  96508,
  51394,
  67019,
  21518,
  62995,
  92092,
  10928,
  34981,
  26876,
  66391,
  35901,
  18722,
  59497,
  53275,
  72863,
  72352,
  26468,
  47173,
  26320,
  45747,
  75354,
  92639,
  64080,
  88779,
  29791,
  14969,
  68889,
  43718,
  60142,
  42450,
  35427,
  93679,
  40579,
  77933,
  37363,
  21837,
  35039,
  62179,
  39307,
  41332,
  94358,
  98963,
  54717,
  43654,
  69077,
  74844,
  91597,
  66949,
  81313,
  90912,
  47394,
  57684,
  19355,
  97320,
  51077,
  84716,
  88351,
  92708,
  26477,
  52505,
  58645,
  23594,
  87114,
  38462,
  65339,
  78707,
  15918,
  41210,
  33740,
  50265,
  18061,
  17367,
  11451,
  88878,
  14797,
  62073,
  65570,
  42945,
  23455,
  25840,
  94923,
  64165,
  89639,
  27073,
  64190,
  95406,
  28580,
  58531,
  17091,
  89097,
  15175,
  70127,
  40844,
  32925,
  49275,
  58522,
  97797,
  29582,
  21116,
  23705,
  28596,
  84202,
  81833,
  21602,
  60874,
  79507,
  92289,
  59695,
  30349,
  76097,
  90866,
  94385,
  70221,
  20886,
  93814,
  67792,
  24209,
  46911,
  52145,
  35113,
  10340,
  23363,
  46370,
  92407,
  57569,
  16621,
  44760,
  91961,
  49347,
  77133,
  85477,
  40137,
  78890,
  19791,
  66006,
  82274,
  77644,
  93936,
  47149,
  36490,
  23305,
  54503,
  95877,
  20859,
  94959,
  77232,
  63958,
  75367,
  15010,
  67442,
  19373,
  12449,
  49748,
  52620,
  47647,
  24137,
  90427,
  73412,
  32252,
  29261,
  74693,
  41579,
  31429,
  56097,
  44950,
  43971,
  59469,
  11373,
  12061,
  65372,
  59499,
  13640,
  26832,
  54344,
  65273,
  33991,
  11683,
  58927,
  39325,
  90238,
  40526,
  26038,
  16808,
  13799,
  37248,
  45920,
  85648,
  66092,
  58553,
  63800,
  56780,
  81096,
  30787,
  59633,
  43677,
  79162,
  13632,
  83126,
  10649,
  54696,
  27113,
  32739,
  49396,
  55625,
  57489,
  96581,
  18357,
  50387,
  78519,
  96718,
  91448,
  20052,
  95230,
  22052,
  90604,
  11139,
  35546,
  58323,
  44445,
  94442,
  16048,
  74949,
  81915,
  31606,
  25300,
  41338,
  30274,
  68688,
  31971,
  19565,
  78989,
  39992,
  87227,
  59542,
  47044,
  40969,
  25297,
  74790,
  86721,
  61396,
  66410,
  96524,
  67455,
  49695,
  93222,
  97465,
  98283,
  24511,
  80045,
  97212,
  14409,
  62374,
  85065,
  24675,
  23975,
  17562,
  82898,
  92248,
  88802,
  93090,
  16605,
  56872,
  42331,
  40692,
  64053,
  56691,
  70406,
  74671,
  81878,
  35731,
  97104,
  98067,
  68290,
  82210,
  21560,
  79248,
  37000,
  82588,
  52634,
  90100,
  16029,
  38326,
  60226,
  77086,
  79446,
  97224,
  17675,
  64106,
  85496,
  75485,
  75698,
  24342,
  46880,
  68573,
  46608,
  25059,
  91385,
  82361,
  11563,
  53683,
  85007,
  56710,
  64028,
  73189,
  44895,
  66152,
  72272,
  79283,
  39856,
  62116,
  77850,
  27195,
  29717,
  58456,
  16162,
  94771,
  72874,
  26966,
  28969,
  79145,
  75048,
  29692,
  86495,
  78868,
  48543,
  48252,
  27605,
  38669,
  76442,
  36193,
  73037,
  34757,
  91312,
  69099,
  44500,
  61860,
  64698,
  39869,
  93116,
  30369,
  65899,
  68519,
  24048,
  47972,
  53891,
  54451,
  31280,
  20399,
  45783,
  98775,
  48390,
  93583,
  96206,
  12588,
  15698,
  97712,
  38580,
  34395,
  23049,
  24820,
  56864,
  46690,
  77603,
  46878,
  47171,
  46708,
  50525,
  50282,
  55230,
  32552,
  41714,
  35644,
  67623,
  56606,
  85977,
  52701,
  60629,
  17680,
  25372,
  98212,
  40568,
  41527,
  10528,
  44728,
  40026,
  15934,
  12465,
  40634,
  59014,
  68197,
  18082,
  75040,
  45384,
  41916,
  35128,
  80754,
  31342,
  27303,
  64827,
  99513,
  41314,
  19671,
  30923,
  45018,
  79548,
  55411,
  47707,
  50875,
  27914,
  19353,
  97123,
  30579,
  51476,
  13574,
  79359,
  48141,
  43241,
  48136,
  87448,
  61514,
  42728,
  83862,
  71949,
  79924,
  20640,
  16549,
  52489,
  89623,
  72593,
  23289,
  82803,
  11727,
  61214,
  12199,
  19169,
  73807,
  11622,
  25058,
  28775,
  91583,
  55536,
  94095,
  72890,
  62775,
  70295,
  41751,
  56993,
  82031,
  74057,
  17518,
  99629,
  61336,
  43975,
  81289,
  69367,
  40283,
  61911,
  95657,
  39067,
  67250,
  23292,
  88887,
  85949,
  38715,
  40612,
  87534,
  70873,
  85930,
  89567,
  93731,
  52695,
  14578,
  46609,
  59861,
  14621,
  20328,
  17390,
  68295,
  41982,
  17348,
  72246,
  90096,
  19205,
  90078,
  90572,
  78173,
  99825,
  84869,
  97111,
  46975,
  82586,
  11832,
  70440,
  13421,
  90891,
  70480,
  65072,
  88516,
  47360,
  87714,
  85563,
  58866,
  46273,
  15260,
  45102,
  29064,
  88846,
  90349,
  98819,
  30367,
  41951,
  86574,
  25619,
  79116,
  57943,
  26147,
  75102,
  87742,
  66894,
  45617,
  23600,
  89356,
  68062,
  57149,
  79451,
  47353,
  74094,
  79034,
  86009,
  81225,
  31607,
  12367,
  50512,
  65420,
  17083,
  80423,
  30678,
  53711,
  52826,
  94443,
  12923,
  19993,
  41316,
  43188,
  41603,
  41116,
  47492,
  52271,
  82016,
  11995,
  87664,
  60087,
  77487,
  77742,
  99837,
  68435,
  62077,
  80241,
  82671,
  81232,
  46669,
  32390,
  49245,
  40533,
  84643,
  39991,
  30313,
  59426,
  47142,
  71211,
  59935,
  31958,
  32153,
  86833,
  52808,
  40778,
  79022,
  87874,
  72182,
  39448,
  97758,
  81407,
  80992,
  19276,
  46590,
  10895,
  80793,
  63442,
  30847,
  32880,
  59288,
  96900,
  63642,
  33550,
  72357,
  97040,
  40378,
  59435,
  31818,
  69483,
  93889,
  11266,
  56221,
  66149,
  96835,
  63931,
  24339,
  95811,
  74048,
  66188,
  20544,
  40662,
  99514,
  71983,
  78773,
  10964,
  56979,
  93887,
  34552,
  27038,
  69694,
  24942,
  18028,
  34653,
  90330,
  98296,
  53814,
  89964,
  37589,
  10909,
  98527,
  32976,
  33686,
  79495,
  45397,
  79339,
  48269,
  67186,
  76245,
  84154,
  84888,
  33764,
  62796,
  42861,
  78612,
  13839,
  40455,
  52976,
  55173,
  47339,
  29374,
  23406,
  84137,
  61630,
  89925,
  80845,
  88731,
  10621,
  23613,
  22775,
  82644,
  55188,
  13213,
  46020,
  38894,
  68114,
  39872,
  84638,
  92596,
  66910,
  63454,
  37266,
  22257,
  89661,
  63655,
  63891,
  85248,
  22476,
  80842,
  10278,
  96407,
  13688,
  20190,
  16581,
  44118,
  44289,
  83096,
  78920,
  30294,
  12953,
  58065,
  80257,
  65705,
  36010,
  12082,
  57211,
  37556,
  70791,
  83423,
  69501,
  75688,
  19823,
  38253,
  38183,
  32520,
  65756,
  62670,
  17780,
  39071,
  90304,
  23602,
  40739,
  46361,
  15902,
  37784,
  30916,
  23707,
  91290,
  33864,
  93983,
  35469,
  21070,
  83225,
  81426,
  83168,
  20409,
  57527,
  49574,
  36663,
  60394,
  87952,
  87075,
  48587,
  69841,
  50428,
  61418,
  99136,
  66778,
  23040,
  79526,
  37251,
  67189,
  47448,
  90585,
  98236,
  63822,
  19611,
  99334,
  78062,
  67694,
  29196,
  85703,
  62167,
  93337,
  77737,
  96370,
  42996,
  23203,
  72739,
  59099,
  13739,
  56865,
  72378,
  86826,
  18269,
  57660,
  47959,
  36489,
  50097,
  13746,
  48963,
  30101,
  83534,
  59841,
  69877,
  92081,
  49205,
  48937,
  16871,
  80567,
  64382,
  36178,
  55868,
  46804,
  76304,
  38942,
  56622,
  67077,
  22630,
  73652,
  24594,
  40607,
  24466,
  99685,
  41070,
  53109,
  13303,
  90567,
  63113,
  69961,
  71621,
  48447,
  39391,
  31318,
  54449,
  95458,
  17168,
  59625,
  33979,
  52341,
  31362,
  17105,
  58391,
  59577,
  56230,
  95001,
  60469,
  54419,
  63827,
  98150,
  33965,
  42272,
  27833,
  82723,
  50442,
  63005,
  80894,
  90215,
  93510,
  11301,
  76924,
  96481,
  65294,
  36656,
  28496,
  14320,
  46083,
  22802,
  98104,
  67972,
  10720,
  83626,
  67343,
  96312,
  31279,
  26421,
  84257,
  16680,
  20670,
  62741,
  59245,
  67916,
  89880,
  88768,
  33728,
  64155,
  17490,
  97853,
  21580,
  50349,
  17014,
  95346,
  15226,
  26310,
  50644,
  25134,
  79187,
  21594,
  59975,
  14723,
  59988,
  44916,
  25459,
  99883,
  91474,
  50240,
  90665,
  41306,
  20093,
  77543,
  50058,
  84976,
  25891,
  46484,
  97473,
  86274,
  38121,
  15406,
  74418,
  14474,
  59783,
  37351,
  33242,
  32795,
  86292,
  14090,
  31394,
  17591,
  22263,
  17909,
  37774,
  67198,
  25288,
  48203,
  99738,
  18706,
  54532,
  18588,
  79083,
  86349,
  93820,
  18344,
  80025,
  32006,
  59686,
  10486,
  55016,
  57123,
  72794,
  87491,
  79579,
  65429,
  31765,
  73697,
  91743,
  21682,
  69890,
  72614,
  88539,
  45679,
  41030,
  40402,
  32675,
  40058,
  10228,
  67908,
  32413,
  73185,
  68526,
  49242,
  80390,
  94376,
  47392,
  91154,
  58956,
  81713,
  41037,
  80016,
  62931,
  75384,
  47227,
  99463,
  53299,
  49977,
  49678,
  54480,
  50457,
  53723,
  99495,
  41074,
  90818,
  26932,
  49941,
  16356,
  76996,
  11524,
  30416,
  91352,
  24977,
  49451,
  21400,
  87568,
  96017,
  90873,
  91037,
  75674,
  11356,
  66871,
  33330,
  66278,
  18682,
  42394,
];

pub const RIGHT: [i32; 1000] = [
  48069,
  97906,
  47355,
  59861,
  25840,
  70621,
  57288,
  60361,
  83949,
  40059,
  90866,
  18758,
  96272,
  75644,
  20514,
  95346,
  16581,
  98590,
  73793,
  82185,
  14797,
  90866,
  55469,
  60333,
  40778,
  54922,
  16871,
  76794,
  72739,
  70980,
  95346,
  28152,
  99447,
  17518,
  24820,
  72739,
  91005,
  59416,
  53464,
  39605,
  70333,
  94760,
  29540,
  24339,
  95657,
  66328,
  37044,
  32006,
  24596,
  54211,
  99513,
  92955,
  58645,
  90108,
  32006,
  52479,
  87105,
  22158,
  79034,
  86986,
  23049,
  87991,
  29183,
  10737,
  95657,
  30736,
  60629,
  58645,
  49695,
  49541,
  51537,
  60629,
  10528,
  17518,
  72102,
  95275,
  81328,
  95346,
  11050,
  85720,
  42541,
  90479,
  97931,
  39134,
  43734,
  56231,
  74486,
  19552,
  65123,
  98281,
  59861,
  53722,
  99513,
  24339,
  84766,
  17518,
  12294,
  58543,
  36663,
  40778,
  68002,
  68295,
  41338,
  23115,
  93136,
  68295,
  39868,
  17518,
  57673,
  72739,
  30310,
  78574,
  63049,
  49668,
  71228,
  25233,
  66017,
  74772,
  52133,
  56951,
  95086,
  41338,
  14188,
  17518,
  83068,
  26824,
  15180,
  37974,
  49353,
  53723,
  44740,
  49695,
  65374,
  97310,
  49695,
  10601,
  23998,
  67360,
  24820,
  21859,
  66071,
  31362,
  35818,
  52898,
  73118,
  16871,
  99513,
  34172,
  27153,
  26229,
  77054,
  50353,
  31362,
  24820,
  62375,
  79010,
  43188,
  68295,
  47089,
  57531,
  94109,
  60322,
  17518,
  96684,
  56864,
  59264,
  16750,
  11605,
  24339,
  16871,
  43188,
  45936,
  26176,
  82456,
  58020,
  91408,
  69890,
  92008,
  32006,
  59648,
  96033,
  46048,
  60750,
  39856,
  12788,
  21570,
  76677,
  19313,
  43522,
  58645,
  91689,
  16581,
  95657,
  52033,
  86765,
  91644,
  71159,
  91743,
  20631,
  67879,
  91750,
  37351,
  29318,
  72408,
  19390,
  95346,
  99520,
  63316,
  58645,
  82429,
  86285,
  61266,
  19293,
  43188,
  43188,
  83818,
  53368,
  74844,
  20522,
  74844,
  26932,
  61210,
  24339,
  60629,
  23964,
  56444,
  68400,
  46458,
  16871,
  60629,
  32006,
  69248,
  25946,
  41121,
  97243,
  89900,
  68295,
  72739,
  58645,
  69890,
  53723,
  45624,
  49703,
  45059,
  57910,
  72739,
  91743,
  70945,
  77933,
  60452,
  36663,
  99513,
  95657,
  37351,
  59861,
  58645,
  16581,
  59794,
  16871,
  14797,
  39856,
  88656,
  60629,
  43188,
  58100,
  37192,
  91061,
  39856,
  78213,
  21104,
  13069,
  37351,
  56283,
  37351,
  69763,
  72739,
  74265,
  62276,
  23345,
  60629,
  44246,
  63017,
  40778,
  91743,
  35082,
  19741,
  73790,
  78345,
  37351,
  63953,
  87534,
  69702,
  90866,
  43333,
  25840,
  16581,
  41338,
  46437,
  91163,
  47108,
  26252,
  37351,
  68295,
  50705,
  68295,
  87392,
  25840,
  99513,
  22859,
  79034,
  54120,
  66287,
  24820,
  51395,
  11684,
  58645,
  63197,
  91495,
  44104,
  15212,
  60629,
  39856,
  95868,
  94252,
  98068,
  39400,
  44986,
  36663,
  24378,
  77933,
  61825,
  75051,
  27830,
  56372,
  76831,
  30051,
  65366,
  35697,
  31362,
  72739,
  36663,
  65405,
  80206,
  36663,
  31362,
  14797,
  29177,
  19056,
  68295,
  17518,
  16871,
  75003,
  84372,
  10606,
  95346,
  95346,
  24339,
  62058,
  93381,
  62639,
  50583,
  43188,
  17199,
  16384,
  12954,
  59861,
  16886,
  34718,
  45773,
  28085,
  30318,
  72739,
  42059,
  24685,
  68295,
  48669,
  13857,
  63441,
  14051,
  69040,
  58645,
  44485,
  62062,
  66867,
  15967,
  67958,
  17518,
  60629,
  40778,
  14797,
  58645,
  12810,
  69890,
  99767,
  49978,
  47067,
  68295,
  97859,
  91743,
  35470,
  12131,
  69890,
  56864,
  18029,
  68295,
  99513,
  22289,
  18925,
  39856,
  77629,
  32310,
  53723,
  52273,
  26932,
  33095,
  95346,
  98396,
  40642,
  39714,
  50384,
  72739,
  24820,
  27333,
  95346,
  44497,
  20288,
  72739,
  19177,
  17111,
  39228,
  29286,
  68295,
  94446,
  70031,
  24491,
  90788,
  59861,
  93540,
  35079,
  74844,
  90248,
  42545,
  37351,
  99836,
  40778,
  84050,
  53723,
  78424,
  60629,
  14729,
  95346,
  31177,
  42147,
  14797,
  26567,
  82587,
  30058,
  72739,
  92087,
  16581,
  30154,
  73678,
  74844,
  50232,
  40778,
  89970,
  92795,
  36518,
  93756,
  39856,
  60629,
  60317,
  91987,
  90866,
  14797,
  58645,
  22979,
  40444,
  98418,
  52557,
  25840,
  93195,
  39856,
  46295,
  95346,
  95058,
  58645,
  14797,
  16248,
  16871,
  36713,
  68864,
  96581,
  42649,
  38199,
  16871,
  16581,
  36663,
  89941,
  30182,
  59402,
  31362,
  20465,
  41338,
  41124,
  48272,
  60629,
  54184,
  24853,
  90866,
  74844,
  24820,
  59878,
  96457,
  17518,
  42832,
  24339,
  16581,
  47998,
  20085,
  99513,
  72739,
  30648,
  24339,
  37351,
  73175,
  16581,
  16581,
  60629,
  14180,
  87579,
  29532,
  95571,
  63579,
  11022,
  41296,
  59569,
  40778,
  53723,
  36150,
  59438,
  23878,
  75997,
  40778,
  89703,
  56864,
  68295,
  63214,
  16581,
  24339,
  79034,
  68295,
  16581,
  53723,
  91572,
  96581,
  39856,
  40778,
  91792,
  72739,
  16871,
  37351,
  37528,
  54751,
  98390,
  59349,
  25145,
  21137,
  95346,
  23049,
  16901,
  99513,
  20739,
  37491,
  68336,
  62885,
  40778,
  95346,
  64387,
  54356,
  60629,
  41398,
  99513,
  18128,
  16871,
  67766,
  99513,
  23816,
  37011,
  92255,
  14797,
  74758,
  43188,
  88213,
  49695,
  82407,
  92976,
  78503,
  40778,
  43795,
  24339,
  84821,
  95657,
  45077,
  51112,
  40778,
  69890,
  51194,
  15697,
  79034,
  34441,
  90746,
  53146,
  96439,
  10528,
  73396,
  22834,
  42827,
  25840,
  27279,
  16581,
  11829,
  41338,
  58300,
  51351,
  77063,
  59624,
  69890,
  96581,
  37351,
  39856,
  43188,
  99907,
  17518,
  59861,
  88701,
  41338,
  71349,
  77839,
  16871,
  16581,
  49695,
  96581,
  57537,
  33645,
  99513,
  17342,
  91358,
  61419,
  28119,
  37351,
  96581,
  53294,
  86426,
  96581,
  90866,
  99759,
  16581,
  51210,
  89139,
  89821,
  22488,
  52016,
  61720,
  95346,
  51066,
  70350,
  87604,
  13727,
  41338,
  99513,
  49695,
  31362,
  75693,
  24339,
  16684,
  60875,
  19909,
  55448,
  68295,
  85572,
  43188,
  22924,
  96447,
  43590,
  35446,
  14492,
  33854,
  81254,
  35231,
  69890,
  16581,
  58645,
  18254,
  39765,
  49695,
  44794,
  97117,
  72472,
  69809,
  50894,
  81145,
  16735,
  61819,
  66134,
  67405,
  22869,
  87534,
  40778,
  41656,
  79885,
  96068,
  33841,
  54993,
  60629,
  39856,
  90866,
  99513,
  95346,
  31534,
  58645,
  33154,
  72739,
  17518,
  12393,
  24339,
  69840,
  55306,
  69069,
  17518,
  87753,
  17518,
  40225,
  42636,
  99099,
  60611,
  60629,
  78860,
  30176,
  24339,
  14434,
  30459,
  49695,
  69540,
  41338,
  48918,
  64740,
  55739,
  77933,
  42596,
  15230,
  40778,
  41338,
  55126,
  64249,
  30866,
  15532,
  91743,
  36885,
  23898,
  69375,
  40778,
  52391,
  22592,
  30041,
  17027,
  33410,
  18537,
  30111,
  93883,
  58645,
  36476,
  80458,
  13575,
  18247,
  46338,
  26537,
  17518,
  80947,
  21685,
  95346,
  96979,
  42316,
  49695,
  43002,
  39334,
  95346,
  95122,
  53723,
  17518,
  73195,
  80333,
  53999,
  18047,
  50055,
  13416,
  78838,
  99399,
  72739,
  71044,
  49695,
  48607,
  58645,
  47637,
  31362,
  26932,
  10374,
  99587,
  16871,
  95967,
  48026,
  45844,
  32483,
  91914,
  24079,
  95346,
  38867,
  58038,
  30015,
  51612,
  47825,
  79034,
  69890,
  12438,
  25163,
  57006,
  13374,
  77943,
  95866,
  90866,
  99513,
  89128,
  55488,
  32006,
  78894,
  60629,
  93358,
  35728,
  78590,
  53373,
  18242,
  52358,
  16871,
  29081,
  11880,
  85310,
  41660,
  68295,
  58645,
  20952,
  53723,
  82851,
  68673,
  68295,
  45432,
  39856,
  98886,
  39856,
  24339,
  37351,
  45562,
  99513,
  74844,
  85831,
  32937,
  21022,
  19227,
  95346,
  98417,
  37351,
  51840,
  54835,
  31362,
  37351,
  37351,
  58367,
  59627,
  72739,
  10528,
  92967,
  43188,
  68295,
  68295,
  59861,
  36085,
  81212,
  88554,
  76634,
  95657,
  73103,
  53723,
  17518,
  38109,
  72739,
  40778,
  41338,
  40778,
  84251,
  27789,
  91503,
  68176,
  60629,
  35979,
  69890,
  59861,
  76925,
  39631,
  12043,
  57253,
  80209,
  29891,
  85969,
  27806,
  27607,
  94227,
  67502,
  47087,
  39856,
  72113,
  25501,
  74844,
  91565,
  35641,
  58645,
  68295,
  60629,
  69890,
  31362,
  41744,
  69890,
  52503,
  74591,
  33314,
  97011,
  98273,
  38719,
  24339,
  50497,
  53723,
  24980,
  16581,
  17518,
  97390,
  37461,
  12585,
  19372,
  32006,
  73733,
  91552,
  74844,
  57436,
  58645,
  16871,
  36615,
  12311,
  63638,
  84874,
  95657,
  41338,
  58645,
  77933,
  90866,
  98634,
  45892,
  85888,
  46343,
  53723,
  40778,
  11574,
  32006,
  40778,
  68975,
  49695,
  26569,
  50082,
  41338,
  56029,
  38257,
  72739,
  74844,
  71563,
  37996,
  56074,
  85522,
  90866,
  61520,
  49695,
  89237,
  39529,
  37175,
  95346,
  68295,
  47664,
  99513,
  75031,
  37296,
  86365,
  96190,
  72272,
  36663,
];
