# taplo
A TOML toolkit written in Rust
! [TOML Logosu] (logos / toml-200.png)

TOML v1.0.0-rc.3
================

Tom'un Açık, Minimal Dili.

Tom Preston-Werner, Pradyun Gedam ve diğerleri tarafından.

Hedefler
----------

TOML, aşağıdakilerden dolayı okunması kolay, minimum yapılandırma dosyası formatı olmayı amaçlamaktadır.
bariz anlambilim. TOML, bir karma tabloya açık bir şekilde eşlemek için tasarlanmıştır. TOML
veri yapılarına çok çeşitli dillerde ayrıştırılması kolay olmalıdır.

İçindekiler
-----------------

- [Teknik] (# özellik)
- [Yorum] (# yorum)
- [Anahtar / Değer Çifti] (# anahtar değer çifti)
- [Anahtarlar] (# tuş)
- [Dize] (# dize)
- [Tamsayı] (# tamsayı)
- [Float] (# float)
- [Boolean] (# boole)
- [Ofset Tarih-Saat] (# ofset-tarih-saat)
- [Yerel Tarih-Saat] (# yerel-tarih-saat)
- [Yerel Tarih] (# yerel tarih)
- [Yerel Saat] (# yerel saat)
- [Dizi] (# dizi)
- [Tablo] (# tablo)
- [Satır İçi Tablo] (# satır içi tablo)
- [Tablo Dizisi] (# tablo dizisi)
- [Dosya Adı Uzantısı] (# dosya adı-uzantısı)
- [MIME Türü] (# mime türü)
- [ABNF Dilbilgisi] (# abnf-grammar)

Teknik Özellikler
----

* TOML büyük / küçük harfe duyarlıdır.
* TOML dosyası geçerli bir UTF-8 kodlu Unicode belgesi olmalıdır.
* Boşluk, sekme (0x09) veya boşluk (0x20) anlamına gelir.
* Newline, LF (0x0A) veya CRLF (0x0D 0x0A) anlamına gelir.

Yorum Yap
-------

Karma sembolü, satırın geri kalanını bir yorum olarak işaretler;
dize.

toml
# Bu tam satır bir yorumdur
key = "value" # Bu, satırın sonundaki bir açıklamadır
other = "# Bu bir yorum değil"
''

Sekme dışındaki kontrol karakterleri (U + 0000 - U + 0008, U + 000A - U + 001F, U + 007F)
yorumlarda izin verilmez.

Anahtar / Değer Çifti
--------------

Bir TOML belgesinin birincil yapı taşı anahtar / değer çiftidir.

Anahtarlar eşittir işaretinin solundadır ve değerler sağdadır. Beyaz boşluk
anahtar isimleri ve değerleri etrafında yok sayılır. Anahtar, eşittir işareti ve değer olmalıdır
aynı satırda (bazı değerler birden çok satıra bölünebilir).

toml
anahtar = "değer"
''

Değerler aşağıdaki türlerden birine sahip olmalıdır.

- [Dize] (# dize)
- [Tamsayı] (# tamsayı)
- [Float] (# float)
- [Boolean] (# boole)
- [Ofset Tarih-Saat] (# ofset-tarih-saat)
- [Yerel Tarih-Saat] (# yerel-tarih-saat)
- [Yerel Tarih] (# yerel tarih)
- [Yerel Saat] (# yerel saat)
- [Dizi] (# dizi)
- [Satır İçi Tablo] (# satır içi tablo)

Belirtilmemiş değerler geçersizdir.

toml
anahtar = # GEÇERSİZ
''

Bir anahtar / değer çiftinden sonra bir satırsonu (veya EOF) olmalıdır. (Bkz. [Satır İçi
Tablo] (# inline-table) istisnalar için.)

''
ilk = "Tom" son = "Preston-Werner" # GEÇERSİZ
''

Anahtarlar
----

Bir anahtar boş, alıntı veya noktalı olabilir.

** Çıplak anahtarlar ** yalnızca ASCII harfleri, ASCII rakamları, alt çizgiler ve
kısa çizgiler (`A-Za-z0-9_-`). Çıplak anahtarların yalnızca şunlardan oluşmasına izin verildiğini unutmayın:
ASCII rakamları, örneğin "1234", ancak her zaman dizeler olarak yorumlanır.

toml
anahtar = "değer"
bare_key = "değer"
çıplak anahtar = "değer"
1234 = "değer"
''

** Alıntılanmış anahtarlar ** temel dizeler veya değişmez değerler ile tamamen aynı kuralları izler
dizeler ve çok daha geniş bir anahtar adları kümesi kullanmanıza izin verir. En iyi uygulama
kesinlikle gerekli olmadıkça çıplak anahtarlar kullanmak.

toml
"127.0.0.1" = "değer"
"karakter kodlaması" = "değer"
"ʎǝʞ" = "değer"
'anahtar2' = "değer"
'alıntılanan "değer"' = "değer"
''

Çıplak bir anahtar boş olmamalıdır, ancak boş alıntılanmış bir anahtara izin verilir (ancak
cesareti kırılmış).

toml
= "anahtar adı yok" # GEÇERSİZ
"" = "boş" # GEÇERLİ, ancak önerilmez
'' = 'boş' # VALID, ancak önerilmiyor
''

** Noktalı tuşlar **, bir nokta ile birleştirilmiş bir dizi çıplak veya tırnaklı tuşlardır. Bu
benzer özelliklerin birlikte gruplanmasına izin verir:

toml
name = "Turuncu"
Physical.color = "turuncu"
Physical.shape = "round"
site. "google.com" = doğru
''

JSON arazisinde bu size aşağıdaki yapıyı verir:

json
{
  "name": "Turuncu",
  "fiziksel": {
    "turuncu renk",
    "şekil": "yuvarlak"
  },
  "site": {
    "google.com": doğru
  }
}
''

Noktayla ayrılmış kısımların etrafındaki boşluk yok sayılır, ancak en iyi uygulama,
herhangi bir gereksiz boşluk kullanmayın.

Bir anahtarı birden çok kez tanımlamak geçersizdir.

''
# BUNU YAPMA
name = "Tom"
name = "Pradyun"
''

Çıplak anahtarların ve alıntılanan anahtarların eşdeğer olduğunu unutmayın:

''
# BU ÇALIŞMAYACAK
yazım = "favori"
"yazım" = "favori"
''

Bir anahtar doğrudan tanımlanmadığı sürece, ona hala yazabilir ve
içindeki isimlere.

''
# Bu, "meyve" anahtarını bir sofra haline getirir.
fruit.apple.smooth = true

# Öyleyse masaya "meyveyi" şöyle ekleyebilirsiniz:
meyve.orange = 2
''

''
# AŞAĞIDAKİ GEÇERSİZ

# Bu, fruit.apple'ın değerini bir tamsayı olarak tanımlar.
meyve.apple = 1

# Ama bu meyve, elma gibi davranır.
# Bir tamsayıyı tabloya dönüştüremezsiniz.
fruit.apple.smooth = true
''

Noktalı anahtarların sıra dışı tanımlanması önerilmez.

toml
# GEÇERLİ ANCAK SÖYLEŞİ

apple.type = "meyve"
orange.type = "meyve"

apple.skin = "ince"
orange.skin = "kalın"

apple.color = "kırmızı"
orange.color = "turuncu"
''

toml
# ÖNERİLEN

apple.type = "meyve"
apple.skin = "ince"
apple.color = "kırmızı"

orange.type = "meyve"
orange.skin = "kalın"
orange.color = "turuncu"
''

Çıplak anahtarların yalnızca ASCII tam sayılarından oluşmasına izin verildiğinden, bu mümkündür
kayan harflere benzeyen ancak 2 parçalı noktalı tuşlar olan noktalı tuşlar yazmak için. Yapma
bunu yapmak için iyi bir nedeniniz yoksa (muhtemelen yok).

toml
3.14159 = "pi"
''

Yukarıdaki TOML, aşağıdaki JSON ile eşleşir.

json
{"3": {"14159": "pi"}}
''

Dize
------

Dizeleri ifade etmenin dört yolu vardır: temel, çok satırlı temel, değişmez ve
çok satırlı değişmez. Tüm dizeler yalnızca geçerli UTF-8 karakterleri içermelidir.

** Temel dizeler ** tırnak işaretleri ("" `) ile çevrilidir. Herhangi bir Unicode karakteri
kaçınılması gerekenler dışında kullanılabilir: tırnak işareti, ters eğik çizgi ve
sekme dışındaki kontrol karakterleri (U + 0000 - U + 0008, U + 000A - U + 001F,
U + 007F).

toml
str = "Ben bir dizeyim. \" Bana alıntı yapabilirsiniz \ ". İsim \ tJos \ u00E9 \ nKonum \ tSF."
''

Kolaylık sağlamak için, bazı popüler karakterlerin kompakt bir kaçış dizisi vardır.

''
\ b - geri tuşu (U + 0008)
\ t - sekme (U + 0009)
\ n - satır besleme (U + 000A)
\ f - form beslemesi (U + 000C)
\ r - satır başı (U + 000D)
\ "- tırnak (U + 0022)
\\ - ters eğik çizgi (U + 005C)
\ uXXXX - unicode (U + XXXX)
\ UXXXXXXXX - unicode (U + XXXXXXXX)
''

Herhangi bir Unicode karakteri, "\ uXXXX" veya "\ UXXXXXXXX" biçimleriyle önlenebilir.
Kaçış kodları geçerli Unicode [skalar
değerler] (http://unicode.org/glossary/#unicode_scalar_value).

Yukarıda listelenmeyen diğer tüm kaçış dizileri saklıdır; kullanılırlarsa, TOML
bir hata üretmelidir.

Bazen metin bölümlerini (ör. Çeviri dosyaları) ifade etmeniz gerekir veya
çok uzun bir dizeyi birden çok satıra bölmek ister. TOML bunu kolaylaştırır.

** Çok satırlı temel dizeler ** her birinin üzerinde üç tırnak işareti ile çevrilidir
yan ve yeni satırlara izin verin. Açılış sınırlayıcısının hemen ardından yeni bir satır
kırpılacak. Diğer tüm boşluk ve satırsonu karakterleri değişmeden kalır.

toml
str1 = "" "
Güller kırmızıdır
Menekşeler mavidir"""
''

TOML ayrıştırıcıları, satırsonu satırını normalleştirmekte özgür hissetmelidir.
platformları.

toml
# Bir Unix sisteminde, yukarıdaki çok satırlı dizi büyük olasılıkla aşağıdakilerle aynı olacaktır:
str2 = "Güller kırmızıdır \ nMenekşeler mavidir"

# Bir Windows sisteminde, büyük olasılıkla şuna eşdeğer olacaktır:
str3 = "Güller kırmızıdır \ r \ nMenekşeler mavidir"
''

Gereksiz boşluklar eklemeden uzun dizeler yazmak için bir "satır" kullanın
ters eğik çizgi ". Bir satırdaki son boşluk olmayan karakter bir
çıkış karaktersiz `\`, tüm boşluklarla birlikte (yeni satırlar dahil) kırpılacaktır
sonraki boşluk olmayan karaktere veya kapatma sınırlayıcıya kadar. Tüm kaçış
Temel dizeler için geçerli olan diziler, çok satırlı temel için de geçerlidir.
Teller.

toml
# Aşağıdaki dizeler bayt-bayt eşdeğeridir:
str1 = "Hızlı kahverengi tilki tembel köpeğin üzerinden atlar."

str2 = "" "
Hızlı kahverengi \


  tilki atlar \
    tembel köpek. "" "

str3 = "" "\
       Hızlı kahverengi \
       tilki atlar \
       tembel köpek.
       "" "
''

Kaçılması gerekenler dışında herhangi bir Unicode karakteri kullanılabilir: ters eğik çizgi
ve sekme, satır besleme ve satır başı dışındaki kontrol karakterleri
(U + 0000 - U + 0008, U + 000B, U + 000C, U + 000E - U + 001F, U + 007F).

Herhangi bir yere bir tırnak işareti veya iki bitişik tırnak işareti yazabilirsiniz
çok satırlı bir temel dize içinde. Ayrıca, sayfanın hemen içine de yazılabilirler.
sınırlayıcılar.

toml
str4 = "" "İşte iki tırnak işareti:" ". Yeterince basit." ""
# str5 = "" "İşte üç tırnak işareti:" "". "" "# GEÇERSİZ
str5 = "" "İşte üç tırnak işareti:" "\". "" "
str6 = "" "İşte on beş tırnak işareti:" "\" "" \ "" "\" "" \ "" "\". "" "

# "Bu," dedi, "sadece anlamsız bir ifade."
str7 = "" "" Bu, "dedi," sadece anlamsız bir ifade. "" ""
''

Sık sık Windows yolları veya normal ifadeler belirleyiciyseniz, o zaman
ters eğik çizgilerden hızla kaçmak zorunda kalmak sıkıcı ve hataya açık hale gelir. Yardım etmek,
TOML, kaçmaya hiç izin vermeyen değişmez dizeleri destekler.

** Değişmez dizeler ** tek tırnak içine alınır. Temel dizeler gibi
tek bir satırda görünmelidir:

toml
# Ne görürsen onu alırsın.
winpath = 'C: \ Kullanıcılar \ nodejs \ şablonları'
winpath2 = '\\ ServerX \ admin $ \ system32 \'
alıntı = 'Tom "Dubs" Preston-Werner'
regex = '<\ i \ c * \ s *>'
''

Kaçış olmadığından, tek bir alıntı yazmanın bir yolu yoktur.
tek tırnak içine alınmış değişmez dize. Neyse ki, TOML çok satırlı bir
bu sorunu çözen değişmez dizelerin sürümü.

** Çok satırlı değişmez dizeler ** her birinde üç tek tırnak ile çevrilidir
yan ve yeni satırlara izin verin. Değişmez dizeler gibi, hiçbir şekilde kaçış yoktur.
Açılış sınırlayıcısını hemen takip eden yeni satır kırpılacaktır. Herşey
sınırlayıcılar arasındaki diğer içerik, değiştirilmeden olduğu gibi yorumlanır.

toml
regex2 = '' '[dw] elmaya \ d {2} ihtiyacım yok' ''
çizgiler = '' '
İlk satırsonu
işlenmemiş dizelerde kırpılmış.
   Diğer tüm boşluklar
   Korundu.
'' '
''

Çok satırlı değişmez bir dize içinde herhangi bir yere 1 veya 2 tek tırnak yazabilirsiniz,
ancak üç veya daha fazla tek tırnak dizisine izin verilmez.

toml
quot15 = '' 'İşte on beş tırnak işareti: "" "" "" "" "" "" "" ""' ''

# apos15 = '' 'İşte on beş kesme işareti:' '' '' '' '' '' '' '' '' '# GEÇERSİZ
apos15 = "İşte on beş kesme işareti: '' '' '' '' '' '' '' '"

# "Bu," dedi, "hala anlamsız."
str = '' '' Bu, 'dedi,' hala anlamsız. '' ''
''

Bir hazır bilgi dizesinde sekme dışındaki kontrol karakterlerine izin verilmez. Böylece,
ikili veriler için Base64 veya başka bir uygun ASCII kullanmanız önerilir.
veya UTF-8 kodlaması. Bu kodlamanın işlenmesi uygulamaya özel olacaktır.

Tamsayı
-------

Tam sayılar tam sayılardır. Pozitif sayıların önüne artı işareti eklenebilir.
Negatif sayılar bir eksi işareti ile başlar.

toml
int1 = +99
int2 = 42
int3 = 0
int4 = -17
''

Büyük sayılar için, sayıları geliştirmek için basamaklar arasında alt çizgi kullanabilirsiniz.
okunabilirlik. Her alt çizgi, her birinin üzerinde en az bir rakamla çevrelenmelidir
yan.

toml
int5 = 1_000
int6 = 5_349_221
int7 = 53_49_221 # Hint sayı sistemi gruplaması
int8 = 1_2_3_4_5 # VALID, ancak önerilmez
''

Baştaki sıfırlara izin verilmez. Tam sayı değerleri "-0" ve "+ 0" geçerlidir ve
öneksiz sıfır ile aynı.

Negatif olmayan tamsayı değerleri ayrıca onaltılık, sekizlik veya
ikili. Bu biçimlerde baştaki "+" 'ya izin verilmez ve baştaki sıfırlar
izin verilir (önekten sonra). Hex değerleri büyük / küçük harfe duyarlıdır. Alt çizgiler
rakamlar arasında izin verilir (ancak önek ile değer arasında değil).

toml
# "0x" önekiyle onaltılık
hex1 = 0xDEADBEEF
hex2 = 0xdeadbeef
hex3 = 0xdead_beef

"0o" ön ekiyle # sekizlik
oct1 = 0o01234567
oct2 = 0o755 # Unix dosya izinleri için yararlı

# "0b" ön ekli ikili
bin1 = 0b11010110
''

Keyfi 64 bitlik işaretli tamsayılar (-2 ^ 63'ten 2 ^ 63−1'e kadar) kabul edilmelidir ve
kayıpsız ele alındı. Bir tamsayı kayıpsız olarak temsil edilemiyorsa, bir hata
atılmalıdır.

Yüzer
-----

Kayanlar, IEEE 754 ikili64 değerleri olarak uygulanmalıdır.

Bir kayan nokta, ondalık sayı ile aynı kuralları izleyen bir tam sayı bölümünden oluşur.
tamsayı değerleri), ardından kesirli kısım ve / veya üslü kısım. Her ikisi de bir
kesirli kısım ve üslü kısım mevcuttur, kesirli kısım önce gelmelidir
üs kısmı.

toml
# kesirli
flt1 = +1.0
flt2 = 3.1415
flt3 = -0.01

# üs
flt4 = 5e + 22
flt5 = 1e06
flt6 = -2E-2

# her ikisi de
flt7 = 6.626e-34
''

Kesirli bölüm, bir ondalık nokta ve ardından gelen bir veya daha fazla rakamdır.

Üslü kısım E (büyük veya küçük harf) ve ardından bir tamsayı kısmıdır.
(ondalık tamsayı değerleriyle aynı kuralları izleyen ancak baştaki
sıfırlar).

Kullanılıyorsa ondalık nokta her iki yanında en az bir rakamla çevrelenmelidir.

''
# GEÇERSİZ FLATLAR
geçersiz_float_1 = .7
geçersiz_float_2 = 7.
geçersiz_float_3 = 3.e + 20
''

Tam sayılara benzer şekilde, okunabilirliği artırmak için alt çizgi kullanabilirsiniz. Her biri
alt çizgi en az bir rakamla çevrelenmelidir.

toml
flt8 = 224_617.445_991_228
''

Kayan değerler "-0.0" ve "+ 0.0" geçerlidir ve IEEE 754'e göre eşleşmelidir.

Özel float değerleri de ifade edilebilir. Her zaman küçük harflidirler.

toml
# sonsuz
sf1 = inf # pozitif sonsuz
sf2 = + inf # pozitif sonsuz
sf3 = -inf # negatif sonsuz

# sayı değil
sf4 = nan # gerçek sNaN / qNaN kodlaması uygulamaya özgüdür
sf5 = + nan # "nan" ile aynı
sf6 = -nan # geçerli, gerçek kodlama uygulamaya özeldir
''

Boole
-------

Booleanlar, alışkın olduğunuz belirteçlerdir. Her zaman küçük harf.

toml
bool1 = true
bool2 = yanlış
''

Ofset Tarih-Saat
----------------

Zaman içinde belirli bir anı net bir şekilde temsil etmek için, bir
[RFC 3339] (http://tools.ietf.org/html/rfc3339) ofset ile biçimlendirilmiş tarih-saat.

toml
odt1 = 1979-05-27T07: 32: 00Z
odt2 = 1979-05-27T00: 32: 00-07: 00
odt3 = 1979-05-27T00: 32: 00.999999-07: 00
''

Okunabilirlik açısından tarih ve tarih arasında T sınırlayıcısını değiştirebilirsiniz.
boşluk karakterli zaman (RFC 3339 bölüm 5.6'da izin verildiği gibi).

toml
odt4 = 1979-05-27 07: 32: 00Z
''

Milisaniye hassasiyet gereklidir. Kesirli saniyelerin daha fazla hassasiyeti
uygulamaya özgü. Değer, daha büyük bir kesinlik içeriyorsa
uygulama destekleyebilir, ek hassasiyet kesilmelidir, değil
yuvarlak.

Yerel Tarih-Saat
---------------

Bir [RFC 3339] 'dan uzaklığı çıkarırsanız (http://tools.ietf.org/html/rfc3339)
biçimlendirilmiş tarih-saat, herhangi bir ilişki olmaksızın verilen tarih-saati temsil edecek
bir ofset veya saat dilimine. Olmadan zaman içinde bir ana dönüştürülemez.
ek bilgi. Anına dönüştürme, gerekirse,
uygulamaya özgü.

toml
ldt1 = 1979-05-27T07: 32: 00
ldt2 = 1979-05-27T00: 32: 00.999999
''

Milisaniye hassasiyet gereklidir. Kesirli saniyelerin daha fazla hassasiyeti
uygulamaya özgü. Değer, daha büyük bir kesinlik içeriyorsa
uygulama destekleyebilir, ek hassasiyet kesilmelidir, değil
yuvarlak.

Yerel Tarih
----------

Bir sayfanın yalnızca tarih kısmını eklerseniz
[RFC 3339] (http://tools.ietf.org/html/rfc3339) biçimlendirilmiş tarih-saat,
bir fark veya saat dilimi ile herhangi bir ilişkisi olmaksızın tüm günü temsil eder.

toml
ld1 = 1979-05-27
''

Yerel zaman
----------

Bir [RFC'nin yalnızca zaman bölümünü eklerseniz
3339] (http://tools.ietf.org/html/rfc3339) biçimlendirilmiş tarih-saat, temsil edecek
belirli bir gün veya herhangi bir denkleştirme olmaksızın günün o saati veya
saat dilimi.

toml
lt1 = 07:32:00
lt2 = 00: 32: 00.999999
''

Milisaniye hassasiyet gereklidir. Kesirli saniyelerin daha fazla hassasiyeti
uygulamaya özgü. Değer, daha büyük bir kesinlik içeriyorsa
uygulama destekleyebilir, ek hassasiyet kesilmelidir, değil
yuvarlak.

Dizi
-----

Diziler, içinde değerler bulunan köşeli parantezlerdir. Boşluk yok sayılır. Elementler
virgülle ayrılır. Diziler, aşağıdakilerle aynı veri türlerinin değerlerini içerebilir:
anahtar / değer çiftlerinde izin verilir. Farklı türlerdeki değerler karıştırılabilir.

toml
tamsayılar = [1, 2, 3]
renkler = ["kırmızı", "sarı", "yeşil"]
nested_array_of_int = [[1, 2], [3, 4, 5]]
nested_mixed_array = [[1, 2], ["a", "b", "c"]]
string_array = ["tümü", 'dizeler', "" "aynı" "", '' 'tür' '']

# Karışık tip dizilere izin verilir
sayılar = [0.1, 0.2, 0.5, 1, 2, 5]
katkıda bulunanlar = [
  "Foo Çubuğu <foo@example.com>",
  {name = "Baz Qux", email = "bazqux@example.com", url = "https://example.com/bazqux"}
]
''

Diziler birden çok satıra yayılabilir. Sonlandırıcı virgül (sondaki virgül olarak da adlandırılır)
dizinin son değerinden sonra izin verilir. Herhangi bir sayıda satırsonu ve
yorumlar değerlerden, virgüllerden ve kapanış ayracından önce gelebilir.

toml
tamsayı2 = [
  1, 2, 3
]

tamsayı3 = [
  1,
  2, # sorun değil
]
''

Tablo
-----

Tablolar (karma tablolar veya sözlükler olarak da bilinir) anahtar / değer koleksiyonlarıdır
çiftler. Bir satırda kendi başlarına köşeli parantez içinde görünürler. Onlara söyleyebilirsin
dizilerden ayrı çünkü diziler yalnızca birer değerdir.

toml
[tablo]
''

Bunun altında ve bir sonraki tabloya veya EOF'ye kadar o tablonun anahtar / değerleridir.
Tablolardaki anahtar / değer çiftlerinin belirli bir sırada olması garanti edilmez.

toml
[tablo 1]
key1 = "bir dizi"
anahtar2 = 123

[Tablo 2]
key1 = "başka bir dize"
anahtar2 = 456
''

Tablolar için adlandırma kuralları, anahtarlarla aynıdır (tanımına bakın)
[Anahtarlar] (# tuş) yukarıda).

toml
[köpek. "tater.man"]
type.name = "pug"
''

JSON arazisinde bu size aşağıdaki yapıyı verir:

json
{"köpek": {"tater.man": {"type": {"name": "pug"}}}}
''

Anahtarın etrafındaki boşluk yok sayılır, ancak en iyi uygulama herhangi bir
yabancı beyaz boşluk.

toml
[abc] # bu en iyi uygulamadır
[def] # [def] ile aynı
[g. h. i] # [ghi] ile aynı
[j. "ʞ". 'l'] # [j. "ʞ" ile aynı. 'l']
''

İstemiyorsanız tüm süper tabloları belirtmenize gerek yoktur. TOML bilir
sizin için nasıl yapılır.

toml
# [x] siz
# [xy] yapma
# [xyz] bunlara ihtiyacım var
[xyzw] # bunun işe yaraması için

[x] # daha sonra bir süper masa tanımlamak sorun değil
''

Boş tablolara izin verilir ve sadece içlerinde anahtar / değer çifti bulunmaz.

Anahtarlar gibi, bir tabloyu birden fazla tanımlayamazsınız. Bunu yapmak geçersizdir.

''
# BUNU YAPMA

[meyve]
apple = "kırmızı"

[meyve]
turuncu = "turuncu"
''

''
# BUNU YAPMAYIN

[meyve]
apple = "kırmızı"

[meyve.apple]
doku = "pürüzsüz"
''

Sıra dışı tabloların tanımlanması önerilmez.

toml
# GEÇERLİ ANCAK SÖYLEŞİ
[meyve.apple]
[hayvan]
[meyve.orange]
''

toml
# ÖNERİLEN
[meyve.apple]
[meyve.orange]
[hayvan]
''

Noktalı tuşlar, her noktanın solundaki her şeyi bir tablo olarak tanımlar. Tablolardan beri
birden fazla tanımlanamaz, bu tür tablolar bir "[tablo]" kullanılarak yeniden tanımlanır
başlığa izin verilmiyor. Aynı şekilde, tabloları yeniden tanımlamak için noktalı tuşları kullanmak
"[tablo]" biçiminde tanımlananlara izin verilmez.

"[Tablo]" formu, ancak, tablolar içindeki alt tabloları tanımlamak için kullanılabilir
noktalı tuşlarla tanımlanır.

toml
[meyve]
apple.color = "kırmızı"
apple.taste.sweet = true

# [fruit.apple] # GEÇERSİZ
# [fruit.apple.taste] # GEÇERSİZ

[fruit.apple.texture] # alt tablolar ekleyebilirsiniz
pürüzsüz = doğru
''

Satır İçi Tablo
------------

Satır içi tablolar, tabloları ifade etmek için daha kompakt bir sözdizimi sağlar. Onlar
özellikle aksi takdirde hızlı bir şekilde ayrıntılı hale gelebilecek gruplanmış veriler için kullanışlıdır.
Satır içi tablolar küme ayraçları içine alınır: `{` ve `}`. Parantez içinde sıfır
veya daha fazla virgülle ayrılmış anahtar / değer çifti görünebilir. Anahtar / değer çiftleri,
standart tablolardaki anahtar / değer çiftleriyle aynı biçim. Tüm değer türlerine izin verilir,
satır içi tablolar dahil.

Satır içi tabloların tek bir satırda görünmesi amaçlanmıştır. Sonlandırıcı bir virgül (ayrıca
sondaki virgül olarak adlandırılır), son anahtar / değer çiftinden sonra izin verilmez
satır içi tablo. Küme parantezleri arasında yeni satıra izin verilmez.
bir değer içinde geçerlidir. Öyle olsa bile, bir satır içi bölümü kırmak kesinlikle önerilmez
tabloyu katlar halinde çizin. Kendinizi bu arzuya kaptırırsanız,
standart tablolar kullanmanız gerektiği anlamına gelir.

toml
isim = {ilk = "Tom", son = "Preston-Werner"}
nokta = {x = 1, y = 2}
hayvan = {type.name = "pug"}
''

Yukarıdaki satır içi tablolar, aşağıdaki standart tablo ile aynıdır
tanımlar:

toml
[isim]
ilk = "Tom"
last = "Preston-Werner"

[nokta]
x = 1
y = 2

[hayvan]
type.name = "pug"
''

Satır içi tablolar, içlerindeki anahtarları ve alt tabloları tam olarak tanımlar. Yeni anahtarlar ve
bunlara alt tablolar eklenemez.

toml
[ürün]
type = {name = "Nail"}
# type.edible = false # GEÇERSİZ
''

Benzer şekilde, satır içi tablolar bir satıra anahtarlar veya alt tablolar eklemek için kullanılamaz.
önceden tanımlanmış tablo.

toml
[ürün]
type.name = "Tırnak"
# type = {edible = false} # GEÇERSİZ
''

Tablo Dizisi
---------------

Henüz açıklanmayan son sözdizimi, tablo dizilerinin yazılmasına izin verir.
Bunlar, çift parantez içinde bir tablo adı kullanılarak ifade edilebilir. Bunun altında ve
bir sonraki tablo veya EOF, bu tablonun anahtar / değerleri olana kadar. Her masada
aynı çift parantez içindeki ad, tablo dizisindeki bir öğe olacaktır. 
tablolar karşılaşılan sırayla eklenir. Olmayan çift parantezli bir masa
herhangi bir anahtar / değer çifti boş bir tablo olarak kabul edilecektir.

toml
[[Ürün:% s]]
name = "Çekiç"
sku = 738594937

[[Ürün:% s]]

[[Ürün:% s]]
name = "Tırnak"
sku = 284758393

color = "gri"
''

JSON arazisinde bu size aşağıdaki yapıyı verecektir.

json
{
  "Ürün:% s": [
    {"name": "Çekiç", "sku": 738594937},
    {},
    {"name": "Tırnak", "sku": 284758393, "renk": "gri"}
  ]
}
''

İç içe geçmiş tablo dizileri de oluşturabilirsiniz. Sadece aynı çift köşeli parantezi kullanın
alt tablolarda sözdizimi. İç içe geçmiş tablo dizilerinde, her biri çift köşeli parantez içinde
alt tablo, en son tanımlanan tablo öğesine ait olacaktır. Normal
alt tablolar (diziler değil) aynı şekilde en son tanımlanan tabloya aittir
öğesi.

toml
[[meyve]]
  name = "elma"

  [fruit.physical] # alt tablo
    color = "kırmızı"
    şekil = "yuvarlak"

  [[fruit.variety]] # iç içe geçmiş tablo dizisi
    name = "kırmızı lezzetli"

  [[fruit.variety]]
    name = "büyükanne demirci"

[[meyve]]
  name = "muz"

  [[fruit.variety]]
    name = "plantain"
''

Yukarıdaki TOML, aşağıdaki JSON ile eşleşir.

json
{
  "meyve": [
    {
      "name": "elma",
      "fiziksel": {
        "kırmızı renk",
        "şekil": "yuvarlak"
      },
      "Çeşitlilik": [
        {"name": "kırmızı lezzetli"},
        {"name": "büyükanne demirci"}
      ]
    },
    {
      "name": "muz",
      "Çeşitlilik": [
        {"name": "plantain"}
      ]
    }
  ]
}
''

Bir tablonun veya tablo dizisinin üst öğesi bir dizi öğesiyse, bu öğe
çocuğun tanımlanabilmesi için önceden tanımlanmış olması gerekir. Girişimde bulunmak
siparişin ayrıştırma zamanında bir hata üretmesi gerektiğini tersine çevirin.

''
# GEÇERSİZ TOML DOC
[fruit.physical] # alt tablo, ancak hangi üst öğeye ait olmalıdır?
  color = "kırmızı"
  şekil = "yuvarlak"

[[meyve]] # ayrıştırıcı, "meyve" nin
           # tablo yerine dizi
  name = "elma"
''

Statik olarak tanımlanmış bir diziye, bu dizi boş olsa bile eklemeye çalışmak,
ayrıştırma zamanında bir hata üretmelidir.

''
# GEÇERSİZ TOML DOC
meyve = []

[[meyve]] # İzin verilmez
''

Önceden kurulmuş bir ile aynı ada sahip normal bir tablo tanımlanmaya çalışılıyor
dizi ayrıştırma zamanında bir hata üretmelidir. Normal bir tabloyu yeniden tanımlamaya çalışmak
bir dizi de aynı şekilde bir ayrıştırma zamanı hatası üretmelidir.

''
# GEÇERSİZ TOML DOC
[[meyve]]
  name = "elma"

  [[fruit.variety]]
    name = "kırmızı lezzetli"

  # GEÇERSİZ: Bu tablo önceki tablo dizisi ile çakışıyor
  [meyve.variety]
    name = "büyükanne demirci"

  [fruit.physical]
    color = "kırmızı"
    şekil = "yuvarlak"

  # GEÇERSİZ: Bu tablo dizisi önceki tabloyla çakışıyor
  [[fruit.physical]]
    color = "yeşil"
''

Ayrıca, uygun olduğunda satır içi tabloları da kullanabilirsiniz:

toml
puan = [{x = 1, y = 2, z = 3},
           {x = 7, y = 8, z = 9},
           {x = 2, y = 4, z = 8}]
''

Dosya Adı Uzantısı
------------------

TOML dosyaları ".toml" uzantısını kullanmalıdır.

MIME Türü
---------

TOML dosyalarını internet üzerinden aktarırken, uygun MIME türü
`application / toml`.

ABNF Dilbilgisi
------------

TOML sözdiziminin resmi bir açıklaması ayrı bir [ABNF dosyası] [abnf] olarak mevcuttur.

[abnf]: https://github.com/toml-lang/toml/blob/1.0.0-rc.3/toml.abnf
