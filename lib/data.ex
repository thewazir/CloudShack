defmodule Data do
  @bands %{
    "2190m" => %{ name: "2190m", start: 0.136, end: 0.137 },
    "560m" => %{ name: "560m", start: 0.501, end: 0.504 },
    "160m" => %{ name: "160m", start: 1.8, end: 2.0 },
    "80m" => %{ name: "80m", start: 3.5, end: 4.0 },
    "60m" => %{ name: "60m", start: 5.102, end: 5.404 },
    "40m" => %{ name: "40m", start: 7.0, end: 7.3 },
    "30m" => %{ name: "30m", start: 10.0, end: 10.15 },
    "20m" => %{ name: "20m", start: 14.0, end: 14.35 },
    "17m" => %{ name: "17m", start: 18.068, end: 18.168 },
    "15m" => %{ name: "15m", start: 21.0, end: 21.45 },
    "12m" => %{ name: "12m", start: 24.890, end: 24.99 },
    "10m" => %{ name: "10m", start: 28.0, end: 29.7 },
    "6m" => %{ name: "6m", start: 50.0, end: 54.0 },
    "4m" => %{ name: "4m", start: 70.0, end: 71.0 },
    "2m" => %{ name: "2m", start: 144.0, end: 148.0 },
    "1.25m" => %{ name: "1.25m", start: 222.0, end: 225.0 },
    "70cm" => %{ name: "70cm", start: 420.0, end: 450.0 },
    "33cm" => %{ name: "33cm", start: 902.0, end: 928.0 },
    "23cm" => %{ name: "23cm", start: 1240.0, end: 1300.0 },
    "13cm" => %{ name: "13cm", start: 2300.0, end: 2450.0 },
    "9cm" => %{ name: "9cm", start: 3300.0, end: 3500.0 },
    "6cm" => %{ name: "6cm", start: 5650.0, end: 5925.0 },
    "3cm" => %{ name: "3cm", start: 10000.0, end: 10500.0 },
    "1.25cm" => %{ name: "1.25cm", start: 24000.0, end: 24250.0 },
    "6mm" => %{ name: "6mm", start: 47000.0, end: 47200.0 },
    "4mm" => %{ name: "4mm", start: 75500.0, end: 81000.0 },
    "2.5mm" => %{ name: "2.5mm", start: 119980.0, end: 120020.0 },
    "2mm" => %{ name: "2mm", start: 142000.0, end: 149000.0 },
    "1mm" => %{ name: "1mm", start: 241000.0, end: 250000.0 }
  }

  @legacy_modes %{
    "AMTORFEC" => %{ mode: "TOR", submode: "AMTORFEC" },
    "ASCI" => %{ mode: "RTTY", submode: "ASCI" },
    "CHIP64" => %{ mode: "CHIP", submode: "CHIP64" },
    "CHIP128" => %{ mode: "CHIP", submode: "CHIP128" },
    "DOMINOF" => %{ mode: "DOMINO", submode: "DOMINOF" },
    "FMHELL" => %{ mode: "HELL", submode: "FMHELL" },
    "FSK31" => %{ mode: "PSK", submode: "FSK31" },
    "GTOR" => %{ mode: "TOR", submode: "GTOR" },
    "HELL80" => %{ mode: "HELL", submode: "HELL80" },
    "HFSK" => %{ mode: "HELL", submode: "HFSK" },
    "JT4A" => %{ mode: "JT4", submode: "JT4A" },
    "JT4B" => %{ mode: "JT4", submode: "JT4B" },
    "JT4C" => %{ mode: "JT4", submode: "JT4C" },
    "JT4D" => %{ mode: "JT4", submode: "JT4D" },
    "JT4E" => %{ mode: "JT4", submode: "JT4E" },
    "JT4F" => %{ mode: "JT4", submode: "JT4F" },
    "JT4G" => %{ mode: "JT4", submode: "JT4G" },
    "JT65A" => %{ mode: "JT65", submode: "JT65A" },
    "JT65B" => %{ mode: "JT65", submode: "JT65B" },
    "JT65C" => %{ mode: "JT65", submode: "JT65C" },
    "MFSK8" => %{ mode: "MFSK", submode: "MFSK8" },
    "MFSK16" => %{ mode: "MFSK", submode: "MFSK16" },
    "PAC2" => %{ mode: "PAC", submode: "PAC2" },
    "PAC3" => %{ mode: "PAC", submode: "PAC3" },
    "PAX2" => %{ mode: "PAX", submode: "PAX2" },
    "PCW" => %{ mode: "CW", submode: "PCW" },
    "PSK10" => %{ mode: "PSK", submode: "PSK10" },
    "PSK31" => %{ mode: "PSK", submode: "PSK31" },
    "PSK63" => %{ mode: "PSK", submode: "PSK63" },
    "PSK63F" => %{ mode: "PSK", submode: "PSK63F" },
    "PSK125" => %{ mode: "PSK", submode: "PSK125" },
    "PSKAM10" => %{ mode: "PSK", submode: "PSKAM10" },
    "PSKAM31" => %{ mode: "PSK", submode: "PSKAM31" },
    "PSKAM50" => %{ mode: "PSK", submode: "PSKAM50" },
    "PSKFEC31" => %{ mode: "PSK", submode: "PSKFEC31" },
    "PSKHELL" => %{ mode: "HELL", submode: "PSKHELL" },
    "QPSK31" => %{ mode: "PSK", submode: "QPSK31" },
    "QPSK63" => %{ mode: "PSK", submode: "QPSK63" },
    "QPSK125" => %{ mode: "PSK", submode: "QPSK125" },
    "THRBX" => %{ mode: "THRB", submode: "THRBX" }
  }

  @dxcc_entities %{
    1 => %{ name: "CANADA", flag: "CA" },
    3 => %{ name: "AFGHANISTAN", flag: "AF" },
    4 => %{ name: "AGALEGA & ST BRANDON", flag: "MP" },
    5 => %{ name: "ALAND IS", flag: "AX" },
    6 => %{ name: "ALASKA", flag: "US" },
    7 => %{ name: "ALBANIA", flag: "AL" },
    8 => %{ name: "ALDABRA", flag: "SC" },
    9 => %{ name: "AMERICAN SAMOA", flag: "AS" },
    10 => %{ name: "AMSTERDAM & ST PAUL", flag: "FR" },
    11 => %{ name: "ANDAMAN & NICOBAR IS", flag: "IN" },
    12 => %{ name: "ANGUILLA", flag: "AI" },
    13 => %{ name: "ANTARCTICA", flag: "AQ" },
    14 => %{ name: "ARMENIA", flag: "AM" },
    15 => %{ name: "ASIATIC RUSSIA", flag: "RU" },
    16 => %{ name: "AUCKLAND & CAMPBELL", flag: "NZ" },
    17 => %{ name: "AVES ISLAND", flag: "VE" },
    18 => %{ name: "AZERBAIJAN", flag: "AZ" },
    19 => %{ name: "BAJO NUEVO", flag: "CO" },
    20 => %{ name: "BAKER, HOWLAND IS", flag: "US" },
    21 => %{ name: "BALEARIC IS", flag: "ES" },
    22 => %{ name: "PALAU", flag: "PW" },
    23 => %{ name: "BLENHEIM REEF", flag: "MU" },
    24 => %{ name: "BOUVET", flag: "NO" },
    25 => %{ name: "BRITISH N. BORNEO", flag: "GB" },
    26 => %{ name: "BRITISH SOMALI", flag: "GB" },
    27 => %{ name: "BELARUS", flag: "BY" },
    29 => %{ name: "CANARY IS", flag: "IC" },
    30 => %{ name: "CELEBE/MOLUCCA IS", flag: "ID" },
    31 => %{ name: "C KIRIBATI", flag: "KI" },
    32 => %{ name: "CEUTA & MELILLA", flag: "ES" },
    33 => %{ name: "CHAGOS", flag: "GB" },
    34 => %{ name: "CHATHAM IS", flag: "NZ" },
    35 => %{ name: "CHRISTMAS IS", flag: "CX" },
    36 => %{ name: "CLIPPERTON IS", flag: "FR" },
    37 => %{ name: "COCOS ISLAND", flag: "CR" },
    38 => %{ name: "COCOS-KEELING IS", flag: "CC" },
    40 => %{ name: "CRETE", flag: "GR" },
    41 => %{ name: "CROZET", flag: "FR" },
    43 => %{ name: "DESECHEO IS", flag: "PR" },
    45 => %{ name: "DODECANESE", flag: "GR" },
    46 => %{ name: "EAST MALAYSIA", flag: "MY" },
    47 => %{ name: "EASTER IS", flag: "CL" },
    48 => %{ name: "EASTERN KIRIBATI", flag: "KI" },
    49 => %{ name: "EQUATORIAL GUINEA", flag: "GQ" },
    50 => %{ name: "MEXICO", flag: "MX" },
    51 => %{ name: "ERITREA", flag: "ER" },
    52 => %{ name: "ESTONIA", flag: "EE" },
    53 => %{ name: "ETHIOPIA", flag: "ET" },
    54 => %{ name: "EUROPEAN RUSSIA", flag: "RU" },
    55 => %{ name: "FARQUHAR", flag: "SC" },
    56 => %{ name: "FERNANDO DE NORONHA", flag: "BR" },
    57 => %{ name: "FRENCH EQ. AFRICA", flag: "FR" },
    58 => %{ name: "FRENCH INDO-CHINA", flag: "FR" },
    59 => %{ name: "FRENCH WEST AFRICA", flag: "FR" },
    60 => %{ name: "BAHAMAS", flag: "BS" },
    61 => %{ name: "FRANZ JOSEF LAND", flag: "RU" },
    62 => %{ name: "BARBADOS", flag: "BB" },
    63 => %{ name: "FRENCH GUIANA", flag: "FR" },
    64 => %{ name: "BERMUDA", flag: "BM" },
    65 => %{ name: "BRITISH VIRGIN IS", flag: "VG" },
    66 => %{ name: "BELIZE", flag: "BZ" },
    67 => %{ name: "FRENCH INDIA", flag: "FR" },
    69 => %{ name: "CAYMAN ISLANDS", flag: "KY" },
    70 => %{ name: "CUBA", flag: "CU" },
    71 => %{ name: "GALAPAGOS", flag: "EC" },
    72 => %{ name: "DOMINICAN REPUBLIC", flag: "DO" },
    74 => %{ name: "EL SALVADOR", flag: "SV" },
    75 => %{ name: "GEORGIA", flag: "GE" },
    76 => %{ name: "GUATEMALA", flag: "GT" },
    77 => %{ name: "GRENADA", flag: "GD" },
    78 => %{ name: "HAITI", flag: "HT" },
    79 => %{ name: "GUADELOUPE", flag: "FR" },
    80 => %{ name: "HONDURAS", flag: "HN" },
    81 => %{ name: "GERMANY", flag: "DE" },
    82 => %{ name: "JAMAICA", flag: "JM" },
    84 => %{ name: "MARTINIQUE", flag: "MQ" },
    86 => %{ name: "NICARAGUA", flag: "NI" },
    88 => %{ name: "PANAMA", flag: "PA" },
    89 => %{ name: "TURKS & CAICOS IS", flag: "TC" },
    90 => %{ name: "TRINIDAD & TOBAGO", flag: "TT" },
    91 => %{ name: "ARUBA", flag: "AW" },
    94 => %{ name: "ANTIGUA & BARBUDA", flag: "AG" },
    95 => %{ name: "DOMINICA", flag: "DM" },
    96 => %{ name: "MONTSERRAT", flag: "MS" },
    97 => %{ name: "ST LUCIA", flag: "LC" },
    98 => %{ name: "ST VINCENT", flag: "VC" },
    99 => %{ name: "GLORIOSO IS", flag: "FR" },
    100 => %{ name: "ARGENTINA", flag: "AR" },
    103 => %{ name: "GUAM", flag: "GU" },
    104 => %{ name: "BOLIVIA", flag: "BO" },
    105 => %{ name: "GUANTANAMO BAY", flag: "US" },
    106 => %{ name: "GUERNSEY", flag: "GG" },
    107 => %{ name: "GUINEA", flag: "GN" },
    108 => %{ name: "BRAZIL", flag: "BR" },
    109 => %{ name: "GUINEA-BISSAU", flag: "GW" },
    110 => %{ name: "HAWAII", flag: "US" },
    111 => %{ name: "HEARD IS", flag: "AU" },
    112 => %{ name: "CHILE", flag: "CL" },
    114 => %{ name: "ISLE OF MAN", flag: "IM" },
    115 => %{ name: "ITALIAN SOMALI", flag: "IT" },
    116 => %{ name: "COLOMBIA", flag: "CO" },
    117 => %{ name: "ITU HQ", flag: "CH" },
    118 => %{ name: "JAN MAYEN", flag: "NO" },
    120 => %{ name: "ECUADOR", flag: "EC" },
    122 => %{ name: "JERSEY", flag: "JE" },
    123 => %{ name: "JOHNSTON IS", flag: "US" },
    124 => %{ name: "JUAN DE NOVA", flag: "FR" },
    125 => %{ name: "JUAN FERNANDEZ", flag: "CL" },
    126 => %{ name: "KALININGRAD", flag: "RU" },
    129 => %{ name: "GUYANA", flag: "GY" },
    130 => %{ name: "KAZAKHSTAN", flag: "KZ" },
    131 => %{ name: "KERGUELEN", flag: "FR" },
    132 => %{ name: "PARAGUAY", flag: "PY" },
    133 => %{ name: "KERMADEC", flag: "NZ" },
    134 => %{ name: "KINGMAN REEF", flag: "US" },
    135 => %{ name: "KYRGYZSTAN", flag: "KG" },
    136 => %{ name: "PERU", flag: "PE" },
    137 => %{ name: "REPUBLIC OF KOREA", flag: "KR" },
    138 => %{ name: "KURE ISLAND", flag: "US" },
    140 => %{ name: "SURINAME", flag: "SR" },
    141 => %{ name: "FALKLAND IS", flag: "FK" },
    142 => %{ name: "LAKSHADWEEP ISLANDS", flag: "IN" },
    143 => %{ name: "LAOS", flag: "LA" },
    144 => %{ name: "URUGUAY", flag: "UY" },
    145 => %{ name: "LATVIA", flag: "LV" },
    146 => %{ name: "LITHUANIA", flag: "LT" },
    147 => %{ name: "LORD HOWE IS", flag: "AU" },
    148 => %{ name: "VENEZUELA", flag: "VE" },
    149 => %{ name: "AZORES", flag: "PT" },
    150 => %{ name: "AUSTRALIA", flag: "AU" },
    152 => %{ name: "MACAO", flag: "MO" },
    153 => %{ name: "MACQUARIE IS", flag: "AU" },
    157 => %{ name: "NAURU", flag: "NR" },
    158 => %{ name: "VANUATU", flag: "VU" },
    159 => %{ name: "MALDIVES", flag: "MV" },
    160 => %{ name: "TONGA", flag: "TO" },
    161 => %{ name: "MALPELO IS", flag: "CO" },
    162 => %{ name: "NEW CALEDONIA", flag: "NC" },
    163 => %{ name: "PAPUA NEW GUINEA", flag: "PG" },
    165 => %{ name: "MAURITIUS IS", flag: "MU" },
    166 => %{ name: "MARIANA IS", flag: "US" },
    167 => %{ name: "MARKET REEF", flag: "SE" },
    168 => %{ name: "MARSHALL IS", flag: "MH" },
    169 => %{ name: "MAYOTTE", flag: "YT" },
    170 => %{ name: "NEW ZEALAND", flag: "NZ" },
    171 => %{ name: "MELLISH REEF", flag: "AU" },
    172 => %{ name: "PITCAIRN IS", flag: "PN" },
    173 => %{ name: "MICRONESIA", flag: "FM" },
    174 => %{ name: "MIDWAY IS", flag: "US" },
    175 => %{ name: "FRENCH POLYNESIA", flag: "PF" },
    176 => %{ name: "FIJI", flag: "FJ" },
    177 => %{ name: "MINAMI TORISHIMA", flag: "JP" },
    179 => %{ name: "MOLDOVA", flag: "MD" },
    180 => %{ name: "MOUNT ATHOS", flag: "GR" },
    181 => %{ name: "MOZAMBIQUE", flag: "MZ" },
    182 => %{ name: "NAVASSA IS", flag: "US" },
    185 => %{ name: "SOLOMON ISLANDS", flag: "SB" },
    187 => %{ name: "NIGER", flag: "NE" },
    188 => %{ name: "NIUE", flag: "NU" },
    189 => %{ name: "NORFOLK IS", flag: "NF" },
    190 => %{ name: "SAMOA", flag: "WS" },
    191 => %{ name: "N COOK IS", flag: "NZ" },
    192 => %{ name: "OGASAWARA", flag: "JP" },
    195 => %{ name: "ANNOBON I.", flag: "GQ" },
    197 => %{ name: "PALMYRA & JARVIS IS", flag: "US" },
    199 => %{ name: "PETER I IS", flag: "NO" },
    201 => %{ name: "PRINCE EDWARD & MARION", flag: "ZA" },
    202 => %{ name: "PUERTO RICO", flag: "PR" },
    203 => %{ name: "ANDORRA", flag: "AD" },
    204 => %{ name: "REVILLAGIGEDO", flag: "MX" },
    205 => %{ name: "ASCENSION ISLAND", flag: "GB" },
    206 => %{ name: "AUSTRIA", flag: "AT" },
    207 => %{ name: "RODRIGUEZ IS", flag: "MU" },
    209 => %{ name: "BELGIUM", flag: "BE" },
    211 => %{ name: "SABLE ISLAND", flag: "CA" },
    212 => %{ name: "BULGARIA", flag: "BG" },
    213 => %{ name: "SAINT MARTIN", flag: "FR" },
    214 => %{ name: "CORSICA", flag: "FR" },
    215 => %{ name: "CYPRUS", flag: "CY" },
    216 => %{ name: "SAN ANDRES & PROVIDENCIA", flag: "NI" },
    217 => %{ name: "SAN FELIX", flag: "CL" },
    219 => %{ name: "SAO TOME & PRINCIPE", flag: "ST" },
    221 => %{ name: "DENMARK", flag: "DK" },
    222 => %{ name: "FAROE IS", flag: "FO" },
    223 => %{ name: "ENGLAND", flag: "england" },
    224 => %{ name: "FINLAND", flag: "FI" },
    225 => %{ name: "SARDINIA", flag: "IT" },
    227 => %{ name: "FRANCE", flag: "FR" },
    229 => %{ name: "GERMAN DEM. REP.", flag: "DE" },
    230 => %{ name: "FED REP OF GERMANY", flag: "DE" },
    232 => %{ name: "SOMALIA", flag: "SO" },
    233 => %{ name: "GIBRALTAR", flag: "GI" },
    234 => %{ name: "S COOK IS", flag: "GS" },
    235 => %{ name: "SOUTH GEORGIA IS", flag: "GS" },
    236 => %{ name: "GREECE", flag: "GR" },
    237 => %{ name: "GREENLAND", flag: "GL" },
    238 => %{ name: "SOUTH ORKNEY IS", flag: "GB" },
    239 => %{ name: "HUNGARY", flag: "HU" },
    240 => %{ name: "SOUTH SANDWICH ISLANDS", flag: "GS" },
    241 => %{ name: "SOUTH SHETLAND ISLANDS", flag: "GB" },
    242 => %{ name: "ICELAND", flag: "IS" },
    245 => %{ name: "IRELAND", flag: "IE" },
    246 => %{ name: "SOV MILITARY ORDER OF MALTA", flag: "MT" },
    247 => %{ name: "SPRATLY IS", flag: "PH" },
    248 => %{ name: "ITALY", flag: "IT" },
    249 => %{ name: "ST KITTS & NEVIS", flag: "KN" },
    250 => %{ name: "ST HELENA IS", flag: "SH" },
    251 => %{ name: "LIECHTENSTEIN", flag: "LI" },
    252 => %{ name: "ST PAUL ISLAND", flag: "CA" },
    253 => %{ name: "ST. PETER & ST. PAUL ROCKS", flag: "BR" },
    254 => %{ name: "LUXEMBOURG", flag: "LU" },
    255 => %{ name: "SINT MAARTEN, SABA, ST EUSTATIUS", flag: "NL" },
    256 => %{ name: "MADEIRA IS", flag: "PT" },
    257 => %{ name: "MALTA", flag: "MT" },
    259 => %{ name: "SVALBARD IS", flag: "NO" },
    260 => %{ name: "MONACO", flag: "MC" },
    262 => %{ name: "TAJIKISTAN", flag: "TJ" },
    263 => %{ name: "NETHERLANDS", flag: "NL" },
    265 => %{ name: "NORTHERN IRELAND", flag: "GB" },
    266 => %{ name: "NORWAY", flag: "NO" },
    269 => %{ name: "POLAND", flag: "PL" },
    270 => %{ name: "TOKELAU IS", flag: "TK" },
    272 => %{ name: "PORTUGAL", flag: "PT" },
    273 => %{ name: "TRINDADE & MARTIN VAZ ISLANDS", flag: "BR" },
    274 => %{ name: "TRISTAN DA CUNHA & GOUGH IS", flag: "GB" },
    275 => %{ name: "ROMANIA", flag: "RO" },
    276 => %{ name: "TROMELIN", flag: "FR" },
    277 => %{ name: "ST PIERRE & MIQUELON", flag: "CA" },
    278 => %{ name: "SAN MARINO", flag: "SM" },
    279 => %{ name: "SCOTLAND", flag: "scotland" },
    280 => %{ name: "TURKMENISTAN", flag: "TM" },
    281 => %{ name: "SPAIN", flag: "ES" },
    282 => %{ name: "TUVALU", flag: "TV" },
    283 => %{ name: "UK BASES ON CYPRUS", flag: "CY" },
    284 => %{ name: "SWEDEN", flag: "SE" },
    285 => %{ name: "US VIRGIN ISLANDS", flag: "VI" },
    286 => %{ name: "UGANDA", flag: "UG" },
    287 => %{ name: "SWITZERLAND", flag: "CH" },
    288 => %{ name: "UKRAINE", flag: "UA" },
    289 => %{ name: "UNITED NATIONS HQ", flag: "united-nations" },
    291 => %{ name: "UNITED STATES", flag: "US" },
    292 => %{ name: "UZBEKISTAN", flag: "UZ" },
    293 => %{ name: "VIETNAM", flag: "VN" },
    294 => %{ name: "WALES", flag: "wales" },
    295 => %{ name: "VATICAN", flag: "VA" },
    296 => %{ name: "SERBIA", flag: "RS" },
    297 => %{ name: "WAKE IS", flag: "US" },
    298 => %{ name: "WALLIS & FUTUNA", flag: "WF" },
    299 => %{ name: "WEST MALAYSIA", flag: "MY" },
    301 => %{ name: "W KIRIBATI", flag: "KI" },
    302 => %{ name: "WESTERN SAHARA", flag: "EH" },
    303 => %{ name: "WILLIS IS", flag: "AU" },
    304 => %{ name: "BAHRAIN", flag: "BH" },
    305 => %{ name: "BANGLADESH", flag: "BD" },
    306 => %{ name: "BHUTAN", flag: "BT" },
    308 => %{ name: "COSTA RICA", flag: "CR" },
    309 => %{ name: "MYANMAR", flag: "MM" },
    312 => %{ name: "CAMBODIA", flag: "KH" },
    315 => %{ name: "SRI LANKA", flag: "LK" },
    318 => %{ name: "CHINA", flag: "CN" },
    321 => %{ name: "HONG KONG", flag: "HK" },
    324 => %{ name: "INDIA", flag: "IN" },
    327 => %{ name: "INDONESIA", flag: "ID" },
    330 => %{ name: "IRAN", flag: "IR" },
    333 => %{ name: "IRAQ", flag: "IQ" },
    336 => %{ name: "ISRAEL", flag: "IL" },
    339 => %{ name: "JAPAN", flag: "JP" },
    342 => %{ name: "JORDAN", flag: "JO" },
    344 => %{ name: "DEMOCRATIC PEOPLE'S REPUBLIC OF KOREA", flag: "KP" },
    345 => %{ name: "BRUNEI", flag: "BN" },
    348 => %{ name: "KUWAIT", flag: "KW" },
    354 => %{ name: "LEBANON", flag: "LB" },
    363 => %{ name: "MONGOLIA", flag: "MN" },
    369 => %{ name: "NEPAL", flag: "NP" },
    370 => %{ name: "OMAN", flag: "OM" },
    372 => %{ name: "PAKISTAN", flag: "PK" },
    375 => %{ name: "PHILIPPINES", flag: "PH" },
    376 => %{ name: "QATAR", flag: "QA" },
    378 => %{ name: "SAUDI ARABIA", flag: "SA" },
    379 => %{ name: "SEYCHELLES", flag: "SC" },
    381 => %{ name: "SINGAPORE", flag: "SG" },
    382 => %{ name: "DJIBOUTI", flag: "DJ" },
    384 => %{ name: "SYRIA", flag: "SY" },
    386 => %{ name: "TAIWAN", flag: "TW" },
    387 => %{ name: "THAILAND", flag: "TH" },
    390 => %{ name: "TURKEY", flag: "TR" },
    391 => %{ name: "UNITED ARAB EMIRATES", flag: "AE" },
    400 => %{ name: "ALGERIA", flag: "DZ" },
    401 => %{ name: "ANGOLA", flag: "AO" },
    402 => %{ name: "BOTSWANA", flag: "BW" },
    404 => %{ name: "BURUNDI", flag: "BI" },
    406 => %{ name: "CAMEROON", flag: "CM" },
    408 => %{ name: "CENTRAL AFRICAN REPUBLIC", flag: "CF" },
    409 => %{ name: "CAPE VERDE", flag: "CV" },
    410 => %{ name: "CHAD", flag: "TD" },
    411 => %{ name: "COMOROS", flag: "KM" },
    412 => %{ name: "REPUBLIC OF THE CONGO", flag: "CG" },
    414 => %{ name: "DEM. REPUBLIC OF THE CONGO", flag: "CD" },
    416 => %{ name: "BENIN", flag: "BJ" },
    420 => %{ name: "GABON", flag: "GA" },
    422 => %{ name: "THE GAMBIA", flag: "GM" },
    424 => %{ name: "GHANA", flag: "GH" },
    428 => %{ name: "COTE D'IVOIRE", flag: "CI" },
    430 => %{ name: "KENYA", flag: "KE" },
    432 => %{ name: "LESOTHO", flag: "LS" },
    434 => %{ name: "LIBERIA", flag: "LR" },
    436 => %{ name: "LIBYA", flag: "LY" },
    438 => %{ name: "MADAGASCAR", flag: "MG" },
    440 => %{ name: "MALAWI", flag: "MW" },
    442 => %{ name: "MALI", flag: "ML" },
    444 => %{ name: "MAURITANIA", flag: "MR" },
    446 => %{ name: "MOROCCO", flag: "MA" },
    450 => %{ name: "NIGERIA", flag: "NG" },
    452 => %{ name: "ZIMBABWE", flag: "ZW" },
    453 => %{ name: "REUNION", flag: "FR" },
    454 => %{ name: "RWANDA", flag: "RW" },
    456 => %{ name: "SENEGAL", flag: "SN" },
    458 => %{ name: "SIERRA LEONE", flag: "SL" },
    460 => %{ name: "ROTUMA IS", flag: "FJ" },
    462 => %{ name: "REPUBLIC OF SOUTH AFRICA", flag: "ZA" },
    464 => %{ name: "NAMIBIA", flag: "NA" },
    466 => %{ name: "SUDAN", flag: "SD" },
    468 => %{ name: "SWAZILAND", flag: "SZ" },
    470 => %{ name: "TANZANIA", flag: "TZ" },
    474 => %{ name: "TUNISIA", flag: "TN" },
    478 => %{ name: "EGYPT", flag: "EG" },
    480 => %{ name: "BURKINA-FASO", flag: "BF" },
    482 => %{ name: "ZAMBIA", flag: "ZM" },
    483 => %{ name: "TOGO", flag: "TG" },
    489 => %{ name: "CONWAY REEF", flag: "FJ" },
    490 => %{ name: "BANABA ISLAND", flag: "KI" },
    492 => %{ name: "YEMEN", flag: "YE" },
    497 => %{ name: "CROATIA", flag: "HR" },
    499 => %{ name: "SLOVENIA", flag: "SI" },
    501 => %{ name: "BOSNIA-HERZEGOVINA", flag: "BA" },
    502 => %{ name: "MACEDONIA", flag: "MK" },
    503 => %{ name: "CZECH REPUBLIC", flag: "CZ" },
    504 => %{ name: "SLOVAK REPUBLIC", flag: "SK" },
    505 => %{ name: "PRATAS IS", flag: "TW" },
    506 => %{ name: "SCARBOROUGH REEF", flag: "PH" },
    507 => %{ name: "TEMOTU PROVINCE", flag: "SB" },
    508 => %{ name: "AUSTRAL IS", flag: "PF" },
    509 => %{ name: "MARQUESAS IS", flag: "FR" },
    510 => %{ name: "PALESTINE", flag: "PS" },
    511 => %{ name: "TIMOR-LESTE", flag: "TL" },
    512 => %{ name: "CHESTERFIELD IS", flag: "GB" },
    513 => %{ name: "DUCIE IS", flag: "PN" },
    514 => %{ name: "MONTENEGRO", flag: "ME" },
    515 => %{ name: "SWAINS ISLAND", flag: "US" },
    516 => %{ name: "ST. BARTHELEMY", flag: "FR" },
    517 => %{ name: "CURACAO", flag: "CW" },
    518 => %{ name: "SINT MAARTEN", flag: "NL" },
    519 => %{ name: "ST EUSTATIUS AND SABA", flag: "AN" },
    520 => %{ name: "BONAIRE", flag: "NL" },
    521 => %{ name: "SOUTH SUDAN", flag: "SS" }
  }

  for {id, dxcc} <- @dxcc_entities do
    def lookup_dxcc(unquote(id)), do: unquote(Macro.escape(dxcc))
  end

  def lookup_dxcc(_), do: nil
end