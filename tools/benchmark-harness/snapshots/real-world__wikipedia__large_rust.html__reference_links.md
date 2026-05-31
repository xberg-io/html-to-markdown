---
canonical: https://en.wikipedia.org/wiki/Rust_(programming_language)
meta-ResourceLoaderDynamicStyles:
meta-format-detection: telephone=no
meta-generator: MediaWiki 1.45.0-wmf.20
meta-og:image: https://upload.wikimedia.org/wikipedia/commons/thumb/d/d5/Rust_programming_language_black_logo.svg/1200px-Rust_programming_language_black_logo.svg.png
meta-og:image:height: 1200
meta-og:image:width: 1200
meta-og:title: Rust (programming language) - Wikipedia
meta-og:type: website
meta-referrer: origin-when-cross-origin
meta-robots: max-image-preview:standard
meta-viewport: width=1120
title: Rust (programming language) - Wikipedia
---

[Jump to content][1]

## Rust (programming language)

51 languages

- [Afrikaans][2]
- [العربية][3]
- [Azərbaycanca][4]
- [বাংলা][5]
- [閩南語 / Bân-lâm-gí][6]
- [Беларуская][7]
- [Català][8]
- [Čeština][9]
- [Dansk][10]
- [Deutsch][11]
- [Eesti][12]
- [Español][13]
- [Esperanto][14]
- [Euskara][15]
- [فارسی][16]
- [Français][17]
- [Galego][18]
- [한국어][19]
- [Hrvatski][20]
- [Bahasa Indonesia][21]
- [Íslenska][22]
- [Italiano][23]
- [עברית][24]
- [Kiswahili][25]
- [Latviešu][26]
- [Lombard][27]
- [Magyar][28]
- [മലയാളം][29]
- [Bahasa Melayu][30]
- [Nederlands][31]
- [日本語][32]
- [Norsk bokmål][33]
- [Polski][34]
- [Português][35]
- [Română][36]
- [Runa Simi][37]
- [Русский][38]
- [Shqip][39]
- [Simple English][40]
- [Slovenčina][41]
- [Српски / srpski][42]
- [Suomi][43]
- [Svenska][44]
- [తెలుగు][45]
- [ไทย][46]
- [Türkçe][47]
- [Українська][48]
- [Tiếng Việt][49]
- [粵語][50]
- [中文][51]
- [Kadazandusun][52]


[Edit links][53]

[/wiki/Wikipedia:Good_articles*][55]

From Wikipedia, the free encyclopedia

General-purpose programming language

| Rust                                                                                                                                                                                                                                                                                               | |
| -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| [/wiki/File:Rust_programming_language_black_logo.svg][57]                                                                                                                                                                                                                                          | |
| [Paradigms][58]                                                                                                                                                                                                                                                                                    | [Concurrent][59] [functional][60] [generic][61] [imperative][62] [structured][63]                                                   |
| [Developer][64]                                                                                                                                                                                                                                                                                    | The Rust Team                                                                                                                       |
| First appeared                                                                                                                                                                                                                                                                                     | January 19, 2012; 13 years ago (2012-01-19)                                                                                         |
|                                                                                                                                                                                                                                                                                                    | |
| [Stable release][65]                                                                                                                                                                                                                                                                               | 1.90.0[[1]][66] [https://www.wikidata.org/wiki/Q575650?uselang=en#P348][68]  / September 18, 2025; 13 days ago (September 18, 2025) |
|                                                                                                                                                                                                                                                                                                    | |
| [Typing discipline][69]                                                                                                                                                                                                                                                                            | [Affine][70] [inferred][71] [nominal][72] [static][73] [strong][74]                                                                 |
| Implementation language                                                                                                                                                                                                                                                                            | [OCaml][75] (2006–2011)   Rust  (2012–present)                                                                                      |
| [Platform][76]                                                                                                                                                                                                                                                                                     | [Cross-platform][77][[note 1]][78]                                                                                                  |
| [OS][79]                                                                                                                                                                                                                                                                                           | [Cross-platform][77][[note 2]][80]                                                                                                  |
| [License][81]                                                                                                                                                                                                                                                                                      | [MIT][82],  [Apache 2.0][83][[note 3]][84]                                                                                          |
| [Filename extensions][85]                                                                                                                                                                                                                                                                          | `.rs`, `.rlib`                                                                                                                      |
| Website                                                                                                                                                                                                                                                                                            | [rust-lang.org][86]                                                                                                                 |
| Influenced by                                                                                                                                                                                                                                                                                      | |
| [Alef][87] [BETA][88] [CLU][89] [C#][90][C++][91] [Cyclone][92] [Elm][93] [Erlang][94][Haskell][95] [Hermes][96] [Limbo][97] [Mesa][98][Napier][99][Newsqueak][100] [NIL][101][[note 4]][102][OCaml][75] [Ruby][103][Sather][104] [Scheme][105][Standard ML][106] [Swift][107][[7]][108][[8]][109] | |
| Influenced                                                                                                                                                                                                                                                                                         | |
| [Idris][110][[9]][111] [Project Verona][112][[10]][113] [SPARK][114][[11]][115] [Swift][107][[12]][116] [V][117][[13]][118] [Zig][119][[14]][120]                                                                                                                                                  | |


**Rust** is a [general-purpose][121] [programming language][122]. It is noted for its emphasis on [performance][123], [type safety][124], [concurrency][125], and [memory safety][126].

Rust supports multiple [programming paradigms][58]. It was influenced by ideas from [functional programming][60], including [immutability][127], [higher-order functions][128], [algebraic data types][129], and [pattern matching][130].
 It also supports [object-oriented programming][131] via structs, [enums][132], traits,
 and methods. Rust is noted for enforcing memory safety (i.e., that all [references][133] point to valid memory) without a conventional [garbage collector][134]; instead, memory safety errors and [data races][135] are prevented by the "borrow checker", which tracks the [object lifetime][136] of
 references [at compile time][137].

Software developer Graydon Hoare created Rust in 2006 while working at [Mozilla][138] Research, which officially
 sponsored the project in 2009. The first stable release, Rust 1.0, was published
 in May 2015. Following a layoff of Mozilla employees in August 2020, four other
 companies joined Mozilla in sponsoring Rust through the creation of the [Rust Foundation][139] in February 2021.

Rust has been adopted by many software projects, especially [web services][140] and [system software][141], and
 is the first language other than [C][142] and [assembly][143] to
 be supported in the development of the [Linux kernel][144]. It has been
 studied academically and has a growing community of developers.

### Etymology

[[edit][145]]

Rust was named for the [group of fungi][146] that are
 "over-engineered for survival".[[15]][147] The Rust logo was developed in 2011 based on a bicycle [chainring][148].[[16]][149]

### History

[[edit][150]]

#### 2006–2009: Early years

[[edit][151]]

[/wiki/File:MozillaCaliforniaHeadquarters.JPG][153]

*Mozilla Foundation headquarters, 650 Castro Street in [Mountain View, California][154], June 2009*

Rust began as a personal project by [Mozilla][138] employee Graydon Hoare in
 2006. [[15]][147] Hoare started the project due to his frustration with a broken elevator in his
 apartment building.[[15]][147] During the time period between 2006 and 2009, Rust was not publicized to others
 at Mozilla and was written in Hoare's free time;[[17]][155]: 7:50 Hoare began speaking about the language around 2009 after a small group at
 Mozilla became interested in the project.[[18]][156] Hoare emphasized prioritizing good ideas from old languages over new
 development, citing languages including [CLU][89] (1974), [BETA][88] (1975), [Mesa][98] (1977), NIL (1983),[[note 5]][157] [Erlang][94] (1987), [Newsqueak][100] (1988), [Napier][99] (1988), [Hermes][96] (1990), [Sather][104] (1990), [Alef][87] (1992), and [Limbo][97] (1996) as influences, stating "many older languages [are] better than new ones",
 and describing the language as "technology from the past come to save the future
 from itself."[[17]][155]: 8:17 [[18]][156] Early Rust developer Manish Goregaokar similarly described Rust as being based
 on "mostly decades-old research."[[15]][147]

During the early years, the Rust [compiler][137] was written in about
 38,000 lines of [OCaml][75].[[17]][155]: 15:34 [[19]][158] Early Rust contained features such as explicit [object-oriented programming][131] via an `obj` keyword (later removed),[[17]][155]: 10:08 and a [typestates][101] system that would allow variables of a type to be tracked along with state
 changes (such as going from uninitialized to initialized, also removed).[[17]][155]: 13:12

#### 2009–2012: Mozilla sponsorship

[[edit][159]]

Mozilla officially sponsored the Rust project in 2009.[[15]][147] [Brendan Eich][160] and other
 executives, intrigued by the possibility of using Rust for a safe [web browser][161] [engine][162], placed
 engineers on the project including Patrick Walton, Niko Matsakis, Felix Klock,
 and Manish Goregaokar.[[15]][147] A conference room taken by the project developers was dubbed "the nerd cave,"
 with a sign placed outside the door.[[15]][147]

During this time period, work had shifted from the initial OCaml compiler to a [self-hosting compiler][163], *i.e.*, written in Rust, based on [LLVM][164].[[20]][165][[note 6]][166] The Rust ownership system was also in place by 2010.[[15]][147]

The first public release, Rust 0.1, was released on January 20, 2012[[22]][167] for Windows, Linux, and MacOS.[[23]][168] The early 2010s saw increasing involvement from open source volunteers outside
 of Mozilla and outside of the United States. At Mozilla, executives would
 eventually employ over a dozen engineers to work on Rust full time over the next
 decade.[[15]][147]

#### 2012–2015: Evolution

[[edit][169]]

The years from 2012 to 2015 were marked by substantial changes to the Rust [type system][69], including the
 removal of the typestate system and [garbage collector][134].[[17]][155]: 18:36 [[15]][147] Memory management through the ownership system was gradually consolidated and
 expanded to prevent memory-related bugs. By 2013, the garbage collector feature
 was rarely used, and was removed by the team in favor of the ownership
 system.[[15]][147] Other changes during this time included the removal of [pure functions][170], which
 were declared by an explicit `pure` annotation, in March 2013.[[24]][171] Specialized syntax support for [channels][172] and various pointer types were removed to simplify the language.[[17]][155]: 22:32

According to Rust developer Steve Klabnik, Rust was influenced during this
 period by developers coming from [C++][91] (e.g., low-level performance of
 features), [scripting languages][173] (e.g., Cargo and package management), and [functional programming][60] (e.g., type systems development).[[17]][155]: 30:50

Graydon Hoare stepped down from Rust in 2013.[[15]][147] After Hoare's departure, it evolved organically under a federated governance
 structure, with a "core team" of initially six people,[[17]][155]: 21:45 around 30-40 developers total across various other teams,[[17]][155]: 22:22 and a Request for Comments (RFC) process for new language features added in
 March 2014.[[17]][155]: 33:47 The core team would grow to nine people by 2016[[17]][155]: 21:45 with over 1600 proposed RFCs.[[17]][155]: 34:08

According to Andrew Binstock writing for *[Dr. Dobb's Journal][174]* in January 2014, while Rust was "widely viewed as a remarkably elegant
 language", adoption slowed because it radically changed from version to
 version.[[25]][175] Rust development at this time was focused on finalizing the language features
 and moving towards 1.0 so it could begin promising [backward compatibility][176].[[17]][155]: 41:26

Six years after Mozilla sponsored its development, the first [stable release][177], Rust 1.0, was published on May 15, 2015.[[15]][147] A year after the release, the Rust compiler had accumulated over 1,400
 contributors and there were over 5,000 third-party libraries published on the
 Rust package management website Crates.io.[[17]][155]: 43:15

#### 2015–2020: Servo and early adoption

[[edit][178]]

[/wiki/File:Home_page_servo_v0.01.png][180]

*Early homepage of Mozilla's [Servo browser engine][181]*

The development of the [Servo browser engine][181] continued in parallel with Rust, jointly funded by Mozilla and [Samsung][182].[[26]][183] The teams behind the two projects worked in close collaboration; new features in
 Rust were tested out by the Servo team, and new features in Servo were used to
 give feedback back to the Rust team.[[17]][155]: 5:41 The first version of Servo was released in 2016.[[15]][147] The [Firefox][184] web browser shipped with
 Rust code as of 2016 (version 45),[[17]][155]: 53:30 [[27]][185] but components of Servo did not appear in Firefox until September 2017 (version
 57) as part of the [Gecko][186] and [Quantum][187] projects.[[28]][188]

Improvements were made to the Rust toolchain ecosystem during the years
 following 1.0 including [Rustfmt][189], [integrated development environment][190] integration,[[17]][155]: 44:56 and a regular compiler testing and release cycle.[[17]][155]: 46:48 Rust also gained a community [code of conduct][191] and
 an [IRC][192] chat for community discussion.[[17]][155]: 50:36

The earliest known adoption outside of Mozilla was by individual projects at
 Samsung, [Facebook][193] (now [Meta Platforms][194]), [Dropbox][195], and Tilde, Inc. (the
 company behind [ember.js][196]).[[17]][155]: 55:44 [[15]][147] [Amazon Web Services][197] followed in 2020.[[15]][147] Engineers acknowledged the risks of adopting a new technology; they cited
 performance, lack of a garbage collector, safety, and pleasantness of working in
 the language as reasons for the adoption. Amazon developers cited a finding by
 Portuguese researchers that Rust code used [less energy][198] compared to similar code written in [Java][199] and [C++][91] (behind only [C][142]).[[15]][147][[29]][200][[note 7]][201]

#### 2020–present: Mozilla layoffs and Rust Foundation

[[edit][202]]

In August 2020, Mozilla laid off 250 of its 1,000 employees worldwide, as part
 of a corporate restructuring caused by the [COVID-19 pandemic][203].[[30]][204][[31]][205] The team behind Servo was disbanded. The event raised concerns about the future
 of Rust.[[32]][206] In the following week, the Rust Core Team acknowledged the severe impact of the
 layoffs and announced that plans for a Rust foundation were underway. The first
 goal of the foundation would be to take ownership of all [trademarks][207] and [domain names][208] and to take
 financial responsibility for their costs.[[33]][209]

On February 8, 2021, the formation of the [Rust Foundation][139] was announced by five founding
 companies: [Amazon Web Services][197], [Google][210], [Huawei][211], [Microsoft][212], and [Mozilla][138].[[34]][213][[35]][214] The foundation, led by Shane Miller for its first two years, offered $20,000
 grants and other support for programmers working on major Rust features.[[15]][147] In a [blog][215] post published on April 6, 2021,
 Google announced support for Rust within the [Android Open Source Project][216] as an alternative to C/C++.[[36]][217]

On November 22, 2021, the Moderation Team, which was responsible for enforcing
 the community code of conduct, announced their resignation "in protest of the
 Core Team placing themselves unaccountable to anyone but themselves".[[37]][218] In May 2022, the Rust Core Team, other lead programmers, and members of the Rust
 Foundation board implemented governance reforms in response to the incident.[[38]][219]

The Rust Foundation posted a draft for a new trademark policy on April 6, 2023,
 which resulted in widespread negative reactions from Rust users and
 contributors.[[39]][220] The trademark policy included rules for how the Rust logo and name could be
 used.[[39]][220]

On February 26, 2024, the U.S. [White House][221] [Office of the National Cyber Director][222] released a 19-page press report urging software development to move away from C
 and C++ to memory-safe languages like C#, Go, Java, Ruby, Swift, and Rust.[[40]][223][[41]][224][[42]][225] News coverage of Rust increased in light of the report.[[43]][226][[44]][227]

### Syntax and features

[[edit][228]]

Main article: [Rust syntax][229]

Rust's [syntax][230] is similar to that of [C][142] and C++,[[45]][231][[46]][232] although many of its features were influenced by [functional programming][60] languages such as [OCaml][75].[[47]][233] Hoare has described Rust as targeted at frustrated C++ developers and emphasized
 features such as safety, control of [memory layout][234], and [concurrency][125].[[18]][156] Safety in Rust includes the guarantees of memory safety, type safety, and lack
 of data races.

#### Hello World program

[[edit][235]]

Below is a ["Hello, World!" program][236] in Rust. The `fn` keyword denotes a [function][237], and the `println!` [macro][238] (see [§ Macros][239]) prints the message to [standard output][240].[[48]][241] [Statements][242] in Rust are separated by [semicolons][243].

```text
fn main() {
    println!("Hello, World!");
}
```

#### Variables

[[edit][244]]

[Variables][245] in Rust are defined through the `let` keyword.[[49]][246] The example below assigns a value to the variable with name `foo` of type `i32` and outputs its value.

```text
fn main() {
    let foo = 10;
    println!("The value of foo is {foo}");
}
```

Variables are [immutable][127] by
 default, but adding the `mut` keyword allows the variable to be mutated.[[50]][247] The following example uses `//`, which denotes the start of a [comment][248].[[51]][249]

```text
fn main() {
    // This code would not compile without adding "mut".
    let mut foo = 10;
    println!("The value of foo is {foo}");
    foo = 20;
    println!("The value of foo is {foo}");
}
```

Multiple `let` expressions can define multiple variables with the same name, known as [variable shadowing][250]. Variable shadowing allows transforming variables without having to name the
 variables differently.[[52]][251] The example below declares a new variable with the same name that is double the
 original value:

```text
fn main() {
    let foo = 10;
    // This will output "The value of foo is 10"
    println!("The value of foo is {foo}");
    let foo = foo * 2;
    // This will output "The value of foo is 20"
    println!("The value of foo is {foo}");
}
```

Variable shadowing is also possible for values of different types. For example,
 going from a string to its length:

```text
fn main() {
    let letters = "abc";
    let letters = letters.len();
}
```

#### Block expressions and control flow

[[edit][252]]

A *block expression* is delimited by [curly brackets][253]. When
 the last expression inside a block does not end with a semicolon, the block
 evaluates to the value of that trailing expression:[[53]][254]

```text
fn main() {
    let x = {
        println!("this is inside the block");
        1 + 2
    };
    println!("1 + 2 = {x}");
}
```

Trailing expressions of function bodies are used as the return value:[[54]][255]

```text
fn add_two(x: i32) -> i32 {
    x + 2
}
```

##### `if` expressions

[[edit][256]]

An `if` [conditional expression][257] executes code based on whether the given value is `true`. `else` can be used for when the value evaluates to `false`, and `else if` can be used for combining multiple expressions.[[55]][258]

```text
fn main() {
    let x = 10;
    if x > 5 {
        println!("value is greater than five");
    }

    if x % 7 == 0 {
        println!("value is divisible by 7");
    } else if x % 5 == 0 {
        println!("value is divisible by 5");
    } else {
        println!("value is not divisible by 7 or 5");
    }
}
```

`if` and `else` blocks can evaluate to a value, which can then be assigned to a variable:[[55]][258]

```text
fn main() {
    let x = 10;
    let new_x = if x % 2 == 0 { x / 2 } else { 3 * x + 1 };
    println!("{new_x}");
}
```

##### `while` loops

[[edit][259]]

`[while][260]` can be used
 to repeat a block of code while a condition is met.[[56]][261]

```text
fn main() {
    // Iterate over all integers from 4 to 10
    let mut value = 4;
    while value <= 10 {
         println!("value = {value}");
         value += 1;
    }
}
```

##### `for` loops and iterators

[[edit][262]]

[For loops][263] in Rust loop over
 elements of a collection.[[57]][264] `for` expressions work over any [iterator][265] type.

```text
fn main() {
    // Using `for` with range syntax for the same functionality as above
    // The syntax 4..=10 means the range from 4 to 10, up to and including 10.
    for value in 4..=10 {
        println!("value = {value}");
    }
}
```

In the above code, `4..=10` is a value of type `Range` which implements the `Iterator` trait. The code within the curly braces is applied to each element returned by
 the iterator.

Iterators can be combined with functions over iterators like `map`, `filter`, and `sum`. For example, the following adds up all numbers between 1 and 100 that are
 multiples of 3:

```text
(1..=100).filter(|&x: i8| -> bool { x % 3 == 0 }).sum()
```

##### `loop` and `break` statements

[[edit][266]]

More generally, the `loop` keyword allows repeating a portion of code until a `break` occurs. `break` may optionally exit the loop with a value. In the case of nested loops, labels
 denoted by `'label_name` can be used to break an outer loop rather than the innermost loop.[[58]][267]

```text
fn main() {
    let value = 456;
    let mut x = 1;
    let y = loop {
        x *= 10;
        if x > value {
            break x / 10;
        }
    };
    println!("largest power of ten that is smaller than or equal to value: {y}");

    let mut up = 1;
    'outer: loop {
        let mut down = 120;
        loop {
            if up > 100 {
                break 'outer;
            }

            if down < 4 {
                break;
            }

            down /= 2;
            up += 1;
            println!("up: {up}, down: {down}");
        }
        up *= 2;
    }
}
```

#### Pattern matching

[[edit][268]]

The `match` and `if let` expressions can be used for [pattern matching][130].
 For example, `match` can be used to double an optional integer value if present, and return zero
 otherwise:[[59]][269]

```text
fn double(x: Option<u64>) -> u64 {
    match x {
        Some(y) => y * 2,
        None => 0,
    }
}
```

Equivalently, this can be written with `if let` and `else`:

```text
fn double(x: Option<u64>) -> u64 {
    if let Some(y) = x {
        y * 2
    } else {
        0
    }
}
```

#### Types

[[edit][270]]

Rust is [strongly typed][271] and [statically typed][272], meaning that the types of all variables must be known at compilation time.
 Assigning a value of a particular type to a differently typed variable causes a [compilation error][273]. [Type inference][71] is
 used to determine the type of variables if unspecified.[[60]][274]

The type `()`, called the "unit type" in Rust, is a concrete type
 that has exactly one value. It occupies no memory (as it represents the absence
 of value). All functions that do not have an indicated return type implicitly
 return `()`. It is similar to `void` in other C-style languages, however `void` denotes the absence of a type and cannot have any value.

The default integer type is `i32`, and the default [floating point][275] type is `f64`. If the type of a [literal][276] number is not explicitly provided, it is either inferred from the context or the
 default type is used.[[61]][277]

##### Primitive types

[[edit][278]]

[Integer types][279] in Rust are named based on the [signedness][280] and the number of
 bits the type takes. For example, `i32` is a signed integer that takes 32 bits of storage, whereas `u8` is unsigned and only takes 8 bits of storage. `isize` and `usize` take storage depending on the [memory address bus width][281] of the compilation target. For example, when building for [32-bit targets][282],
 both types will take up 32 bits of space.[[62]][283][[63]][284]

By default, integer literals are in base-10, but different [radices][285] are supported with prefixes, for
 example, `0b11` for [binary numbers][286], `0o567` for [octals][287], and `0xDB` for [hexadecimals][288]. By
 default, integer literals default to `i32` as its type. Suffixes such as `4u32` can be used to explicitly set the type of a literal.[[64]][289] Byte literals such as `b'X'` are available to represent the [ASCII][290] value (as a `u8`) of a specific character.[[65]][291]

The [Boolean type][292] is referred to as `bool` which can take a value of either `true` or `false`. A `char` takes up 32 bits of space and represents a Unicode scalar value:[[66]][293] a [Unicode codepoint][294] that is not a [surrogate][295].[[67]][296] [IEEE 754][297] floating point numbers
 are supported with `f32` for [single precision floats][298] and `f64` for [double precision floats][299].[[68]][300]

##### Compound types

[[edit][301]]

Compound types can contain multiple values. Tuples are fixed-size lists that can
 contain values whose types can be different. Arrays are fixed-size lists whose
 values are of the same type. Expressions of the tuple and array types can be
 written through listing the values, and can be accessed with `.index` or `[index]`:[[69]][302]

```text
let tuple: (u32, bool) = (3, true);
let array: [i8; 5] = [1, 2, 3, 4, 5];
let value = tuple.1; // true
let value = array[2]; // 3
```

Arrays can also be constructed through copying a single value a number of
 times:[[70]][303]

```text
let array2: [char; 10] = [' '; 10];
```

#### Ownership and references

[[edit][304]]

Rust's ownership system consists of rules that ensure memory safety without
 using a garbage collector. At compile time, each value must be attached to a
 variable called the *owner* of that value, and every value must have
 exactly one owner.[[71]][305] Values are moved between different owners through assignment or passing a value
 as a function parameter. Values can also be *borrowed,* meaning they are
 temporarily passed to a different function before being returned to the
 owner.[[72]][306] With these rules, Rust can prevent the creation and use of [dangling pointers][307]:[[72]][306][[73]][308]

```text
fn print_string(s: String) {
    println!("{}", s);
}

fn main() {
    let s = String::from("Hello, World");
    print_string(s); // s consumed by print_string
    // s has been moved, so cannot be used any more
    // another print_string(s); would result in a compile error
}
```

The function `print_string` takes ownership over the `String` value passed in; Alternatively, `&` can be used to indicate a [reference][133] type (in `&String`) and to create a reference (in `&s`):[[74]][309]

```text
fn print_string(s: &String) {
    println!("{}", s);
}

fn main() {
    let s = String::from("Hello, World");
    print_string(&s); // s borrowed by print_string
    print_string(&s); // s has not been consumed; we can call the function many times
}
```

Because of these ownership rules, Rust types are known as *[linear][310]* or *affine* types, meaning each value can be used exactly once. This
 enforces a form of [software fault isolation][311] as the owner of a value is solely responsible for its correctness and
 deallocation.[[75]][312]

When a value goes out of scope, it is *dropped* by running its [destructor][313]. The destructor may be programmatically defined through implementing the `Drop` [trait][314]. This helps manage resources such as file handles,
 network sockets, and [locks][315], since when objects are dropped, the resources associated with them are closed
 or released automatically.[[76]][316]

##### Lifetimes

[[edit][317]]

[Object lifetime][136] refers to the period of time during which a reference is valid; that is, the
 time between the object creation and destruction.[[77]][318] These *lifetimes* are implicitly associated with all Rust reference types.
 While often inferred, they can also be indicated explicitly with named lifetime
 parameters (often denoted `'a`, `'b`, and so on).[[78]][319]

Lifetimes in Rust can be thought of as [lexically scoped][320], meaning that the duration of an object lifetime is inferred from the set of
 locations in the source code (i.e., function, line, and column numbers) for
 which a variable is valid.[[79]][321] For example, a reference to a local variable has a lifetime corresponding to the
 block it is defined in:[[79]][321]

```text
fn main() {
    let x = 5;                // ------------------+- Lifetime 'a
                              //                   |
    let r = &x;               // -+-- Lifetime 'b  |
                              //  |                |
    println!("r: {}", r);     //  |                |
                              //  |                |
                              // -+                |
}                             // ------------------+
```

The borrow checker in the Rust compiler then enforces that references are only
 used in the locations of the source code where the associated lifetime is
 valid.[[80]][322][[81]][323] In the example above, storing a reference to variable `x` in `r` is valid, as variable `x` has a longer lifetime (`'a`) than variable `r` (`'b`). However, when `x` has a shorter lifetime, the borrow checker would reject the program:

```text
fn main() {
    let r;                    // ------------------+- Lifetime 'a
                              //                   |
    {                         //                   |
        let x = 5;            // -+-- Lifetime 'b  |
        r = &x; // ERROR: x does  |                |
    }           // not live long -|                |
                // enough                          |
    println!("r: {}", r);     //                   |
}                             // ------------------+
```

Since the lifetime of the referenced variable (`'b`) is shorter than the lifetime of the variable holding the reference (`'a`), the borrow checker errors, preventing `x` from being used from outside its scope.[[82]][324]

Lifetimes can be indicated using explicit *lifetime parameters* on function
 arguments. For example, the following code specifies that the reference returned
 by the function has the same lifetime as `original` (and *not* necessarily the same lifetime as `prefix`):[[83]][325]

```text
fn remove_prefix<'a>(mut original: &'a str, prefix: &str) -> &'a str {
    if original.starts_with(prefix) {
        original = original[prefix.len()..];
    }
    original
}
```

In the compiler, ownership and lifetimes work together to prevent memory safety
 issues such as dangling pointers.[[84]][326][[85]][327]

#### User-defined types

[[edit][328]]

User-defined types are created with the `struct` or `enum` keywords. The `struct` keyword is used to denote a [record type][329] that groups multiple related values.[[86]][330] `enum`s can take on different variants at runtime, with its capabilities similar to [algebraic data types][331] found in functional programming languages.[[87]][332] Both records and enum variants can contain [fields][333] with different types.[[88]][334] Alternative names, or aliases, for the same type can be defined with the `type` keyword.[[89]][335]

The `impl` keyword can define methods for a user-defined type. Data and functions are
 defined separately. Implementations fulfill a role similar to that of [classes][336] within other languages.[[90]][337]

##### Standard library

[[edit][338]]

[/wiki/File:Rust_standard_libraries.svg][340]

*A diagram of the dependencies between the standard library modules of Rust.*

The Rust [standard library][341] defines and implements many widely used custom data types, including core data
 structures such as `Vec`, `Option`, and `HashMap`, as well as [smart pointer][342] types.
 Rust also provides a way to exclude most of the standard library using the
 attribute `#![no_std]`, for applications such as embedded devices. Internally, the standard library
 is divided into three parts, `core`, `alloc`, and `std`, where `std` and `alloc` are excluded by `#![no_std]`.[[91]][343]

Rust uses the [option type][344] `Option<T>` to define optional values, which can be matched
 using `if let` or `match` to access the inner value:[[92]][345]

```text
fn main() {
    let name1: Option<&str> = None;
    // In this case, nothing will be printed out
    if let Some(name) = name1 {
        println!("{name}");
    }

    let name2: Option<&str> = Some("Matthew");
    // In this case, the word "Matthew" will be printed out
    if let Some(name) = name2 {
        println!("{name}");
    }
}
```

Similarly, Rust's [result type][346] `Result<T, E>` holds either a successfully computed value (the `Ok` variant) or an error (the `Err` variant)[[93]][347]. Like `Option`, the use of `Result` means that the inner
 value cannot be used directly; programmers must use a `match` expression, syntactic sugar such as `?` (the “try”
 operator), or an explicit `unwrap` assertion to access it. Both `Option` and `Result` are used throughout the standard
 library and are a fundamental part of Rust's explicit approach to handling
 errors and missing data.

#### Pointers

[[edit][348]]

The `&` and `&mut` reference types are guaranteed to not be null and point to valid memory.[[94]][349] The raw pointer types `*const` and `*mut` opt out of the safety guarantees, thus they may be null or invalid; however, it
 is impossible to dereference them unless the code is explicitly declared unsafe
 through the use of an `unsafe` block.[[95]][350] Unlike dereferencing, the creation of raw pointers is allowed inside safe Rust
 code.[[96]][351]

#### Type conversion

[[edit][352]]

This section is an excerpt from [Type conversion § Rust][353].[[edit][354]]

Rust provides no implicit type conversion (coercion) between most primitive
 types. But, explicit type conversion (casting) can be performed using the `as` keyword.[[97]][355]

```text
let x: i32 = 1000;
println!("1000 as a u16 is: {}", x as u16);
```

[//upload.wikimedia.org/wikipedia/commons/transcoded/5/5c/Rust_101.webm/Rust_101.webm.480p.vp9.webm][356]

*A presentation on Rust by Emily Dunham from [Mozilla][138]'s Rust team ([linux.conf.au][357] conference, Hobart, 2017)*

#### Polymorphism

[[edit][358]]

Rust supports [bounded parametric polymorphism][359] through [traits][360] and [generic functions][361].[[98]][362] Common behavior between types may be declared using traits and `impl`s:[[99]][363]

```text
trait Zero: Sized {
    fn zero() -> Self;
    fn is_zero(&self) -> bool
    where
        Self: PartialEq,
    {
        self == &Zero::zero()
    }
}

impl Zero for u32 {
    fn zero() -> u32 { 0 }
}

impl Zero for f32 {
    fn zero() -> Self { 0.0 }
}
```

The example above also includes a method `is_zero` which provides a default implementation that is not required when implementing
 the trait.[[99]][363]

A function can then be made generic by adding type parameters inside angle
 brackets (`<Num>`), which only allow types that implement the trait:

```text
// zero is a generic function with one type parameter, Num
fn zero<Num: Zero>() -> Num {
    Num::zero()
}

fn main() {
    let a: u32 = zero();
    let b: f32 = zero();
    assert!(a.is_zero() && b.is_zero());
}
```

In the examples above, `Num: Zero` as well as `where Self: PartialEq` are trait bounds that constrain the type to only allow types that implement `Zero` or `PartialEq`.[[99]][363] Within a trait or impl, `Self` refers to the type that the code is implementing.[[100]][364]

Generics can be used in functions to allow implementing a behavior for different
 types without repeating the same code. Generic functions can be written in
 relation to other generics, without knowing the actual type.[[101]][365]

##### Trait objects

[[edit][366]]

Generic functions use [static dispatch][367],
 meaning that the type of all parameters that end up being used for the function
 must be known at compile time. Generic functions generate separate copies of the
 code for each combination of generic parameters used in a process called [monomorphization][368].[[102]][369] Because monomorphization duplicates the code for each type used, it is as
 performant as writing functions using concrete types,[[102]][369] but compile time and size of the output binary could be increased.[[103]][370]

However, Rust also uses a feature known as *trait objects* to accomplish [dynamic dispatch][371],
 a type of polymorphism where the implementation of a polymorphic operation is
 chosen at [runtime][372]. This allows for behavior similar to [duck typing][373], where all data
 types that implement a given trait can be treated as functionally
 equivalent.[[104]][374] Trait objects are declared using the syntax `dyn Tr` where `Tr` is a trait. Trait objects are dynamically sized, therefore they
 must be put behind a pointer, such as `Box`.[[105]][375] The following example creates a list of objects where each object can be printed
 out using the `Display` trait:

```text
use std::fmt::Display;

let v: Vec<Box<dyn Display>> = vec![
    Box::new(3),
    Box::new(5.0),
    Box::new("hi"),
];

for x in v {
    println!("{x}");
}
```

If an element in the list does not implement the `Display` trait, it
 will cause a compile-time error.[[106]][376]

#### Memory safety

[[edit][377]]

Rust is designed to be [memory safe][378]. It does not permit null pointers, [dangling pointers][379],
 or [data races][135].[[107]][380][[108]][381][[109]][382][[110]][383] Data values can be initialized only through a fixed set of forms, all of which
 require their inputs to be already initialized.[[111]][384]

#### Memory management

[[edit][385]]

Rust does not use [garbage collection][134]. Memory and other resources are instead managed through the "resource
 acquisition is initialization" convention,[[112]][386] with optional [reference counting][387]. Rust provides deterministic management of resources, with very low [overhead][388].[[113]][389] Values are [allocated on the stack][390] by default, and all [dynamic allocations][391] must be explicit.[[114]][392]

The built-in reference types using the `&` symbol do not involve
 run-time reference counting. The safety and validity of the underlying pointers
 is verified at compile time, preventing [dangling pointers][307] and other forms of [undefined behavior][393].[[115]][394] Rust's type system separates shared, [immutable][127] references of the form `&T` from unique, mutable references of
 the form `&mut T`. A mutable reference can be coerced to an
 immutable reference, but not vice versa.[[116]][395]

#### Unsafe

[[edit][396]]

Rust's memory safety checks may be circumvented through the use of `unsafe` blocks. This allows programmers to dereference arbitrary raw pointers, call
 external code, or perform other low-level functionality not allowed by safe
 Rust.[[117]][397] Some low-level functionality enabled in this way includes [volatile memory access][398], architecture-specific intrinsics, [type punning][399], and inline
 assembly.[[118]][400]

Unsafe code is sometimes needed to implement complex data structures.[[119]][401] A frequently cited example is that it is difficult or impossible to implement [doubly linked lists][402] in safe Rust.[[120]][403][[121]][404][[122]][405][[123]][406]

Programmers using unsafe Rust are considered responsible for upholding Rust's
 memory and type safety requirements, for example, that no two mutable references
 exist pointing to the same location.[[124]][407] If programmers write code which violates these requirements, this results in [undefined behavior][393].[[124]][407] The Rust documentation includes a list of behavior considered undefined,
 including accessing dangling or misaligned pointers, or breaking the aliasing
 rules for references.[[125]][408]

#### Macros

[[edit][409]]

Macros allow generation and transformation of Rust code to reduce repetition.
 Macros come in two forms, with *declarative macros* defined through `macro_rules!`, and *procedural macros*, which are defined in
 separate crates.[[126]][410][[127]][411]

##### Declarative macros

[[edit][412]]

A declarative macro (also called a "macro by example") is a macro, defined using
 the `macro_rules!` keyword, that uses pattern matching to determine
 its expansion.[[128]][413][[129]][414] Below is an example that sums over all its arguments:

```text
macro_rules! sum {
    ( $initial:expr $(, $expr:expr )* $(,)? ) => {
        $initial $(+ $expr)*
    }
}

fn main() {
    let x = sum!(1, 2, 3);
    println!("{x}"); // prints 6
}
```

##### Procedural macros

[[edit][415]]

Procedural macros are Rust functions that run and modify the compiler's input [token][416] stream, before any other components are compiled. They are generally more
 flexible than declarative macros, but are more difficult to maintain due to
 their complexity.[[130]][417][[131]][418]

Procedural macros come in three flavors:

- Function-like macros `custom!(...)`
- Derive macros `#[derive(CustomDerive)]`
- Attribute macros `#[custom_attribute]`


#### Interface with C and C++

[[edit][419]]

Rust supports the creation of [foreign function interfaces][420] (FFI) through the `extern` keyword. A function that uses the C [calling convention][421] can be written using `extern "C" fn`. Symbols can be exported from Rust to other languages through the `#[no_mangle]` attribute, and symbols can be imported into Rust through `extern` blocks:[[note 8]][422][[133]][423]

```text
#[no_mangle]
pub extern "C" fn exported_from_rust(x: i32) -> i32 { x + 1 }
unsafe extern "C" {
    fn imported_into_rust(x: i32) -> i32;
}
```

The `#[repr(C)]` attribute enables deterministic memory layouts for `struct`s and `enum`s for use across FFI boundaries.[[133]][423] External libraries such as `bindgen` and `cxx` can generate Rust bindings for C/C++.[[133]][423][[134]][424]

### Ecosystem

[[edit][425]]

[//upload.wikimedia.org/wikipedia/commons/transcoded/5/52/Cargo_compiling.webm/Cargo_compiling.webm.480p.vp9.webm][426]

*Compiling a Rust program with Cargo*

The Rust ecosystem includes its compiler, its [standard library][427], and additional components for
 software development. Component installation is typically managed by `rustup`, a Rust [toolchain][428] installer
 developed by the Rust project.[[135]][429]

#### Compiler

[[edit][430]]

The Rust compiler, `rustc`, compiles Rust code into [binaries][431]. First, the compiler
 parses the source code into an [AST][432]. Next,
 this AST is lowered to [IR][433]. The compiler backend is then invoked as a subcomponent to apply [optimizations][434] and translate the resulting IR into [object code][435]. Finally, a [linker][436] is used
 to combine the object(s) into a single executable image.[[136]][437]

rustc uses [LLVM][164] as its compiler backend by
 default, but it also supports using alternative backends such as [GCC][438] and [Cranelift][439].[[137]][440] The intention of those alternative backends is to increase platform coverage of
 Rust or to improve compilation times.[[138]][441][[139]][442]

#### Cargo

[[edit][443]]

[/wiki/File:Crates.io_website.png][445]

*Screenshot of crates.io in June 2022*

Cargo is Rust's [build system][446] and [package manager][447].
 It downloads, compiles, distributes, and uploads packages—called *crates*—that are maintained in an official registry. It also acts as a
 front-end for Clippy and other Rust components.[[140]][448]

By default, Cargo sources its dependencies from the user-contributed registry *crates.io*, but [Git][449] repositories,
 crates in the local filesystem, and other external sources can also be specified
 as dependencies.[[141]][450]

#### Rustfmt

[[edit][451]]

Rustfmt is a [code formatter][452] for Rust. It formats whitespace and [indentation][453] to
 produce code in accordance with a common [style][454], unless
 otherwise specified. It can be invoked as a standalone program, or from a Rust
 project through Cargo.[[142]][455]

#### Clippy

[[edit][456]]

[/wiki/File:Cargo_clippy_hello_world_example.png][458]

*Example output of Clippy on a hello world Rust program*

Clippy is Rust's built-in [linting][459] tool to
 improve the correctness, performance, and readability of Rust code. As of
 2024[[update]][460], it has more than 700 rules.[[143]][461][[144]][462]

#### Versioning system

[[edit][463]]

Following Rust 1.0, new features are developed in *nightly* versions which
 are released daily. During each six-week release cycle, changes to nightly
 versions are released to beta, while changes from the previous beta version are
 released to a new stable version.[[145]][464]

Every two or three years, a new "edition" is produced. Editions are released to
 allow making limited [breaking changes][465], such as promoting `await` to a keyword to support [async/await][466] features. Crates
 targeting different editions can interoperate with each other, so a crate can
 upgrade to a new edition even if its callers or its dependencies still target
 older editions. Migration to a new edition can be assisted with automated
 tooling.[[146]][467]

#### IDE support

[[edit][468]]

*rust-analyzer* is a set of [utilities][469] that
 provides [integrated development environments][190] (IDEs) and [text editors][470] with
 information about a Rust project through the [Language Server Protocol][471]. This enables features including [autocomplete][472], and [compilation error][273] display, while editing code.[[147]][473]

### Performance

[[edit][474]]

Since it performs no garbage collection, Rust is often faster than other
 memory-safe languages.[[148]][475][[75]][312][[149]][476] Most of Rust's memory safety guarantees impose no runtime overhead,[[150]][477] with the exception of [array indexing][478] which is checked at runtime by default.[[151]][479] The performance impact of array indexing bounds checks varies, but can be
 significant in some cases.[[151]][479]

Many of Rust's features are so-called *zero-cost abstractions*, meaning
 they are optimized away at compile time and incur no runtime penalty.[[152]][480] The ownership and borrowing system permits [zero-copy][481] implementations for
 some performance-sensitive tasks, such as [parsing][482].[[153]][483] [Static dispatch][367] is
 used by default to eliminate [method calls][484], except for methods called on dynamic trait objects.[[154]][485] The compiler also uses [inline expansion][486] to eliminate [function calls][487] and statically-dispatched method invocations.[[155]][488]

Since Rust uses [LLVM][164], all performance
 improvements in LLVM apply to Rust also.[[156]][489] Unlike C and C++, Rust allows the compiler to reorder struct and enum elements
 unless a `#[repr(C)]` representation attribute is applied.[[157]][490] This allows the compiler to optimize for memory footprint, alignment, and
 padding, which can be used to produce more efficient code in some cases.[[158]][491]

### Adoption

[[edit][492]]

See also: [Category:Rust (programming language) software][493]

[/wiki/File:Firefox_logo,_2019.svg][495]

*[Firefox][184] has components written in
 Rust as part of the underlying [Gecko][186] browser
 engine.*

Rust is used in software across different domains. Components from the Servo
 browser engine (funded by [Mozilla][138] and [Samsung][182]) were incorporated in the [Gecko][186] browser
 engine underlying [Firefox][184].[[159]][496] In January 2023, Google ([Alphabet][497]) announced support for using third party Rust libraries in [Chromium][498].[[160]][499][[161]][500]

Rust is used in several [backend][501] software projects of large [web services][140]. [OpenDNS][502], a [DNS][503] resolution
 service owned by [Cisco][504], uses Rust
 internally.[[162]][505][[163]][506] [Amazon Web Services][197] uses Rust in "performance-sensitive components" of its several services. In
 2019, AWS [open-sourced][507] [Firecracker][508], a virtualization solution primarily written in Rust.[[164]][509] [Microsoft Azure][510] IoT
 Edge, a platform used to run Azure services on [IoT][511] devices,
 has components implemented in Rust.[[165]][512] Microsoft also uses Rust to run containerized modules with [WebAssembly][513] and [Kubernetes][514].[[166]][515] [Cloudflare][516], a company
 providing [content delivery network][517] services, used Rust to build a new [web proxy][518] named Pingora for increased performance and efficiency.[[167]][519] The [npm package manager][520] used Rust for its
 production authentication service in 2019.[[168]][521][[169]][522][[170]][523]

[/wiki/File:Rust_for_Linux_logo.svg][525]

*The [Rust for Linux][526] project has been supported in the [Linux kernel][144] since 2022.*

In operating systems, the [Rust for Linux][526] project, launched in 2020, merged initial support into the [Linux kernel][144] version 6.1
 in late 2022.[[171]][527][[172]][528][[173]][529] The project is active with a team of 6–7 developers, and has added more Rust
 code with kernel releases from 2022 to 2024,[[174]][530] aiming to demonstrate the [minimum viability][531] of the project and resolve key compatibility blockers.[[171]][527][[175]][532] The first drivers written in Rust were merged into the kernel for version
 6.8.[[171]][527] The [Android][533] developers used Rust in 2021 to rewrite existing components.[[176]][534][[177]][535] [Microsoft][212] has rewritten parts of [Windows][536] in
 Rust.[[178]][537] The r9 project aims to re-implement [Plan 9 from Bell Labs][538] in Rust.[[179]][539] Rust has been used in the development of new operating systems such as [Redox][540], a "Unix-like" operating system and [microkernel][541],[[180]][542] Theseus, an experimental operating system with modular state management,[[181]][543][[182]][544] and most of [Fuchsia][545].[[183]][546] Rust is also used for command-line tools and operating system components,
 including [stratisd][547], a [file system][548] manager[[184]][549][[185]][550] and COSMIC, a [desktop environment][551] by [System76][552].[[186]][553]

In web development, [Deno][554], a secure
 runtime for [JavaScript][555] and [TypeScript][556], is built on top of [V8][557] using Rust and Tokio.[[187]][558] Other notable adoptions in this space include [Ruffle][559], an
 open-source [SWF][560] emulator,[[188]][561] and [Polkadot][562], an open source [blockchain][563] and [cryptocurrency][564] platform.[[189]][565]

[Discord][566], an [instant messaging][567] software company, rewrote parts of its system in Rust for increased performance
 in 2020. In the same year, Dropbox announced that its [file synchronization][568] had been rewritten in Rust. [Facebook][193] ([Meta][194]) used Rust to redesign its system that manages source code for internal
 projects.[[15]][147]

In the 2025 [Stack Overflow][569] Developer Survey, 14.8% of respondents had recently done extensive development
 in Rust.[[190]][570] The survey named Rust the "most admired programming language" annually from 2016
 to 2025 (inclusive), as measured by the number of existing developers interested
 in continuing to work in the language.[[191]][571][[note 9]][572] In 2025, 29.2% of developers not currently working in Rust expressed an interest
 in doing so.[[190]][570]

[DARPA][573] has a project TRACTOR (Translating
 All C to Rust) automatically translating C to Rust using techniques such as
 static analysis, dynamic analysis, and large language models.[[192]][574]

### In academic research

[[edit][575]]

Rust's safety and performance have been investigated in [programming language theory][576] research.[[193]][577][[119]][401][[194]][578]

Rust's applicability to writing research software has been examined in other
 fields. A journal article published to *[Proceedings of the International Astronomical Union][579]* used Rust to simulate multi-planet systems.[[195]][580] An article published in *[Nature][581]* shared stories of bioinformaticians using Rust.[[140]][448] Both articles found that Rust has advantages for its performance and safety, and
 cited the [learning curve][582] as
 being a primary drawback to its adoption.

### Community

[[edit][583]]

[/wiki/File:Rustacean-orig-noshadow.svg][585]

*Some Rust users refer to themselves as Rustaceans (similar to the word [crustacean][586]) and have adopted
 an orange crab, Ferris, as their unofficial mascot.[[196]][587][[197]][588]*

According to the *[MIT Technology Review][589]*, the Rust community has been seen as "unusually friendly" to newcomers and
 particularly attracted people from the [queer community][590], partly due to its [code of conduct][191] which outlined a set of expectations for Rust community members to follow.[[15]][147] Inclusiveness of the community has been cited as an important factor for some
 Rust developers.[[140]][448] Demographic data on the community has been collected and published by the Rust
 official blog.[[198]][591]

According to [GitHub][592]'s *State of the Octoverse* project, the Rust community grew by 50.5% in 2022,
 making it one of the fastest growing communities,[[199]][593] though not one of the 10 largest communities as of 2024.[[200]][594]

#### Rust Foundation

[[edit][595]]

*Rust Foundation*

| [/wiki/File:Rust_Foundation_logo.png][597] | |
| ------------------------------------------ | ---------------------------------------------------------------------------------------------- |
| Formation                                  | February 8, 2021; 4 years ago (2021-02-08)                                                     |
| Founders                                   | [Amazon Web Services][197][Google][210][Huawei][211][Microsoft][212] [Mozilla Foundation][598] |
| Type                                       | [Nonprofit organization][599]                                                                  |
| Location                                   | [United States][600]                                                                           |
| [Chairperson][601]                         | Shane Miller                                                                                   |
| [Executive Director][602]                  | Rebecca Rumbul                                                                                 |
| Website                                    | [foundation.rust-lang.org][603]                                                                |


The **Rust Foundation** is a [non-profit][599] [membership organization][604] incorporated in [United States][600], with the
 primary purposes of backing the technical project as a [legal entity][605] and helping to manage the trademark and infrastructure assets.[[201]][606][[46]][232]

It was established on February 8, 2021, with five founding corporate members
 (Amazon Web Services, Huawei, Google, Microsoft, and Mozilla).[[202]][607] The foundation's board is chaired by Shane Miller.[[203]][608] Starting in late 2021, its Executive Director and CEO is Rebecca Rumbul.[[204]][609] Prior to this, Ashley Williams was interim executive director.[[46]][232]

#### Governance teams

[[edit][610]]

The Rust project is composed of *teams* that are responsible for different
 subareas of the development. The compiler team develops, manages, and optimizes
 compiler internals; and the language team designs new language features and
 helps implement them. The Rust project website lists 6 top-level teams as of
 July 2024[[update]][460].[[205]][611] Representatives among teams form the Leadership council, which oversees the Rust
 project as a whole.[[206]][612]

### See also

[[edit][613]]

[/wiki/File:Wikibooks-logo-en-noslogan.svg][615]

Wikibooks has a book on the topic of: ***[Rust for the Novice Programmer][616]***

- [Comparison of programming languages][617]
- [History of programming languages][618]
- [List of programming languages][619]
- [List of programming languages by type][620]
- [List of Rust software and tools][621]


### Notes

[[edit][622]]

1. **[↑][623]** Including build tools, host tools, and standard library support for [x86-64][624], [ARM][625], [MIPS][626], [RISC-V][627], [WebAssembly][513], [i686][628], [AArch64][629], [PowerPC][630], and [s390x][631].[[2]][632]
2. **[↑][633]** Including [Windows][536], [Linux][634], [macOS][635], [FreeBSD][636], [NetBSD][637], and [Illumos][638]. Host build tools on [Android][533], [iOS][639], [Haiku][640], [Redox][540], and [Fuchsia][545] are not officially shipped; these operating systems are supported as

   targets.[[2]][632]

3. **[↑][641]** Third-party dependencies, e.g., [LLVM][164] or [MSVC][642], are

   subject to their own licenses.[[3]][643][[4]][644]

4. **[↑][645]** NIL is cited as an influence for Rust in multiple sources; this likely

   refers to Network Implementation Language developed by Robert Strom and
   others at [IBM][646], which pioneered [typestate analysis][101],[[5]][647][[6]][648] not to be confused with [New Implementation of LISP][649].

5. **[↑][650]** See previous footnote on the referent of NIL.
6. **[↑][651]** The list of Rust compiler versions (referred to as a bootstrapping

   chain) has history going back to 2012.[[21]][652]

7. **[↑][653]** Energy compared to C was 3% more for Rust and 34% more for C++; time

   was 4% more and 56% more, respectively.

8. **[↑][654]** wrapping `no_mangle` with `unsafe` as well as prefacing the `extern "C"` block with `unsafe` are required in the 2024 edition or later.[[132]][655]
9. **[↑][656]** That is, among respondents who have done "extensive development work

   [with Rust] in over the past year" (14.8%), Rust had the largest
   percentage who also expressed interest to "work in [Rust] over the next
   year" (72.4%).[[190]][570]

### References

[[edit][657]]

#### Book sources

[[edit][658]]

- Gjengset, Jon (2021). *Rust for Rustaceans* (1st ed.). No

  Starch Press. [ISBN][659] [9781718501850][660]. [OCLC][661] [1277511986][662].

- Klabnik, Steve; Nichols, Carol (2019-08-12). [*The Rust Programming Language (Covers Rust 2018)*][663]. No Starch Press. [ISBN][659] [978-1-7185-0044-0][664].
- Blandy, Jim; Orendorff, Jason; Tindall, Leonora F. S. (2021). *Programming Rust: Fast, Safe Systems Development* (2nd ed.).

  O'Reilly Media. [ISBN][659] [978-1-4920-5254-8][665]. [OCLC][661] [1289839504][666].

- McNamara, Tim (2021). *Rust in Action*. Manning Publications. [ISBN][659] [978-1-6172-9455-6][667]. [OCLC][661] [1153044639][668].
- Klabnik, Steve; Nichols, Carol (2023). *The Rust programming language* (2nd ed.). No Starch Press. [ISBN][659] [978-1-7185-0310-6][669]. [OCLC][661] [1363816350][670].

#### Others

[[edit][671]]

1. **[↑][672]** ["Announcing Rust 1.90.0"][673]. 2025-09-18. Retrieved 2025-09-18.
2. ^ [***a***][674] [***b***][675] ["Platform Support"][676]. *The rustc book*. [Archived][677] from the original on 2022-06-30. Retrieved 2022-06-27.
3. **[↑][678]** ["Copyright"][679]. *[GitHub][592]*. The Rust Programming Language. 2022-10-19. [Archived][680] from the original on 2023-07-22. Retrieved 2022-10-19.
4. **[↑][681]** ["Licenses"][682]. *The Rust Programming Language*. [Archived][683] from the original on 2025-02-23. Retrieved 2025-03-07.
5. **[↑][684]** Strom, Robert E. (1983). "Mechanisms for compile-time enforcement of

   security". *Proceedings of the 10th ACM SIGACT-SIGPLAN symposium on Principles
   of programming languages - POPL '83*. pp. 276–284. [doi][685]:[10.1145/567067.567093][686]. [ISBN][659] [0897910907][687]. [S2CID][688] [6630704][689].

6. **[↑][690]** Strom, Robert E.; Yemini, Shaula (1986). ["Typestate: A programming language concept for enhancing software reliability"][691] (PDF). *IEEE Transactions on Software Engineering*. **12**. IEEE: 157–171. [doi][685]:[10.1109/tse.1986.6312929][692]. [S2CID][688] [15575346][693].
7. **[↑][694]** ["Uniqueness Types"][695]. *Rust Blog*. [Archived][696] from the original on 2016-09-15. Retrieved 2016-10-08. "Those of you familiar with the Elm style may recognize that the

   updated --explain messages draw heavy
   inspiration from the Elm approach."

8. **[↑][697]** ["Influences"][698]. *The Rust Reference*. [Archived][699] from the original on 2023-11-26. Retrieved 2023-12-31.
9. **[↑][700]** ["Uniqueness Types"][701]. *Idris 1.3.3 documentation*. [Archived][702] from the original on 2018-11-21. Retrieved 2022-07-14. "They are inspired by ... ownership types and borrowed pointers in

   the Rust programming language."

10. **[↑][703]** Tongue, Liam. ["Microsoft opens up Rust-inspired Project Verona programming language on GitHub"][704]. *[ZDNET][705]*. [Archived][706] from the original on 2020-01-17. Retrieved 2020-01-17.
11. **[↑][707]** Jaloyan, Georges-Axel (2017-10-19). "Safe Pointers in SPARK 2014". [arXiv][708]:[1710.07047][709] [[cs.PL][710]].
12. **[↑][711]** Lattner, Chris. ["Chris Lattner's Homepage"][712]. *Nondot*. [Archived][713] from the original on 2018-12-25. Retrieved 2019-05-14.
13. **[↑][714]** ["V documentation (Introduction)"][715]. *[GitHub][592]*. The V Programming Language. Retrieved 2023-11-04.
14. **[↑][716]** Yegulalp, Serdar (2016-08-29). ["New challenger joins Rust to topple C language"][717]. *[InfoWorld][718]*. [Archived][719] from the original on 2021-11-25. Retrieved 2022-10-19.
15. ^ [***a***][720] [***b***][721] [***c***][722] [***d***][723] [***e***][724] [***f***][725] [***g***][726] [***h***][727] [***i***][728] [***j***][729] [***k***][730] [***l***][731] [***m***][732] [***n***][733] [***o***][734] [***p***][735] [***q***][736] [***r***][737] [***s***][738] [***t***][739] Thompson, Clive (2023-02-14). ["How Rust went from a side project to the world's most-loved programming language"][740]. *MIT Technology Review*. [Archived][741] from the original on 2024-09-19. Retrieved 2023-02-23.
16. **[↑][742]** ["Rust logo"][743]. *[Bugzilla][744]*. [Archived][745] from the original on 2024-02-02. Retrieved 2024-02-02.
17. ^ [***a***][746] [***b***][747] [***c***][748] [***d***][749] [***e***][750] [***f***][751] [***g***][752] [***h***][753] [***i***][754] [***j***][755] [***k***][756] [***l***][757] [***m***][758] [***n***][759] [***o***][760] [***p***][761] [***q***][762] [***r***][763] [***s***][764] [***t***][765] [***u***][766] Klabnik, Steve (2016-06-02). ["The History of Rust"][767]. *Applicative 2016*. New York, NY, USA: Association for

    Computing Machinery. p. 80. [doi][685]:[10.1145/2959689.2960081][768]. [ISBN][659] [978-1-4503-4464-7][769].

18. ^ [***a***][770] [***b***][771] [***c***][772] Hoare, Graydon (July 2010). [*Project Servo: Technology from the past come to save the future from itself*][773] (PDF). Mozilla Annual Summit. Archived

    from [the original][774] (PDF) on 2021-12-26. Retrieved 2024-10-29.

19. **[↑][775]** Hoare, Graydon (November 2016). ["Rust Prehistory (Archive of the original Rust OCaml compiler source code)"][776]. *[GitHub][592]*. Retrieved 2024-10-29.
20. **[↑][777]** ["0.1 first supported public release Milestone · rust-lang/rust"][778]. *[GitHub][592]*. Retrieved 2024-10-29.
21. **[↑][779]** Nelson, Jynn (2022-08-05). [*RustConf 2022 - Bootstrapping: The once and future compiler*][780]. Portland, Oregon: Rust Team. Retrieved 2024-10-29 – via YouTube.
22. **[↑][781]** Anderson, Brian (2012-01-24). ["[rust-dev] The Rust compiler 0.1 is unleashed"][782]. *rust-dev* (Mailing list). Archived from [the original][783] on 2012-01-24. Retrieved 2025-01-07.
23. **[↑][784]** Anthony, Sebastian (2012-01-24). ["Mozilla releases Rust 0.1, the language that will eventually usurp Firefox's C++"][785]. *ExtremeTech*. Retrieved 2025-01-07.
24. **[↑][786]** ["Purity by pcwalton · Pull Request #5412 · rust-lang/rust"][787]. *[GitHub][592]*. Retrieved 2024-10-29.
25. **[↑][788]** Binstock, Andrew (2014-01-07). ["The Rise And Fall of Languages in 2013"][789]. *[Dr. Dobb's Journal][174]*. Archived from [the original][790] on 2016-08-07. Retrieved 2022-11-20.
26. **[↑][791]** Lardinois, Frederic (2015-04-03). ["Mozilla And Samsung Team Up To Develop Servo, Mozilla's Next-Gen Browser Engine For Multicore Processors"][792]. *[TechCrunch][793]*. [Archived][794] from the original on 2016-09-10. Retrieved 2017-06-25.
27. **[↑][795]** ["Firefox 45.0, See All New Features, Updates and Fixes"][796]. *Mozilla*. [Archived][797] from the original on 2016-03-17. Retrieved 2024-10-31.
28. **[↑][798]** Lardinois, Frederic (2017-09-29). ["It's time to give Firefox another chance"][799]. *[TechCrunch][793]*. [Archived][800] from the original on 2023-08-15. Retrieved 2023-08-15.
29. **[↑][801]** Pereira, Rui; Couto, Marco; Ribeiro, Francisco; Rua, Rui; Cunha,

    Jácome; Fernandes, João Paulo; Saraiva, João (2017-10-23). ["Energy efficiency across programming languages: How do energy, time, and memory relate?"][802]. [*Proceedings of the 10th ACM SIGPLAN International Conference on Software Language Engineering*][803]. SLE 2017. New York, NY, USA: Association for Computing Machinery.
    pp. 256–267. [doi][685]:[10.1145/3136014.3136031][804]. [hdl][805]:[1822/65359][806]. [ISBN][659] [978-1-4503-5525-4][807].

30. **[↑][808]** Cimpanu, Catalin (2020-08-11). ["Mozilla lays off 250 employees while it refocuses on commercial products"][809]. *[ZDNET][705]*. [Archived][810] from the original on 2022-03-18. Retrieved 2020-12-02.
31. **[↑][811]** Cooper, Daniel (2020-08-11). ["Mozilla lays off 250 employees due to the pandemic"][812]. *[Engadget][813]*. [Archived][814] from the original on 2020-12-13. Retrieved 2020-12-02.
32. **[↑][815]** Tongue, Liam (2020-08-21). ["Programming language Rust: Mozilla job cuts have hit us badly but here's how we'll survive"][816]. *[ZDNET][705]*. [Archived][817] from the original on 2022-04-21. Retrieved 2022-04-21.
33. **[↑][818]** ["Laying the foundation for Rust's future"][819]. *Rust Blog*. 2020-08-18. [Archived][820] from the original on 2020-12-02. Retrieved 2020-12-02.
34. **[↑][821]** ["Hello World!"][822]. *Rust Foundation*. 2020-02-08. [Archived][823] from the original on 2022-04-19. Retrieved 2022-06-04.
35. **[↑][824]** ["Mozilla Welcomes the Rust Foundation"][825]. *Mozilla Blog*. 2021-02-09. [Archived][826] from the original on 2021-02-08. Retrieved 2021-02-09.
36. **[↑][827]** Amadeo, Ron (2021-04-07). ["Google is now writing low-level Android code in Rust"][828]. *Ars Technica*. [Archived][829] from the original on 2021-04-08. Retrieved 2021-04-08.
37. **[↑][830]** Anderson, Tim (2021-11-23). ["Entire Rust moderation team resigns"][831]. *[The Register][832]*. [Archived][833] from the original on 2022-07-14. Retrieved 2022-08-04.
38. **[↑][834]** Levick, Ryan; Bos, Mara. ["Governance Update"][835]. *Inside Rust Blog*. [Archived][836] from the original on 2022-10-27. Retrieved 2022-10-27.
39. ^ [***a***][837] [***b***][838] Claburn, Thomas (2023-04-17). ["Rust Foundation apologizes for trademark policy confusion"][839]. *[The Register][832]*. [Archived][840] from the original on 2023-05-07. Retrieved 2023-05-07.
40. **[↑][841]** Gross, Grant (2024-02-27). ["White House urges developers to dump C and C++"][842]. *[InfoWorld][718]*. Retrieved 2025-01-26.
41. **[↑][843]** Warminsky, Joe (2024-02-27). ["After decades of memory-related software bugs, White House calls on industry to act"][844]. *The Record*. Retrieved 2025-01-26.
42. **[↑][845]** ["Press Release: Future Software Should Be Memory Safe"][846]. [The White House][221].

    2024-02-26. Archived from [the original][847] on 2025-01-18. Retrieved 2025-01-26.

43. **[↑][848]** Jack, Bobby (2024-02-29). ["The White House Wants Memory-Safe Programming, but What Is That?"][849]. *MakeUseOf*. Retrieved 2025-01-26.
44. **[↑][850]** Donovan, Ryan (2024-12-30). ["In Rust we trust? White House Office urges memory safety"][851]. *[The Stack Overflow Blog][569]*. Retrieved 2025-01-26.
45. **[↑][852]** Proven, Liam (2019-11-27). ["Rebecca Rumbul named new CEO of The Rust Foundation"][853]. *[The Register][832]*. [Archived][854] from the original on 2022-07-14. Retrieved 2022-07-14. "Both are curly bracket languages, with C-like syntax that makes

    them unintimidating for C programmers."

46. ^ [***a***][855] [***b***][856] [***c***][857] Vigliarolo, Brandon (2021-02-10). ["The Rust programming language now has its own independent foundation"][858]. *[TechRepublic][859]*. Archived from [the original][860] on 2023-03-20. Retrieved 2022-07-14.
47. **[↑][861]** [Klabnik & Nichols 2019][862],

    p. 263.

48. **[↑][863]** [Klabnik & Nichols 2019][862],

    pp. 5–6.

49. **[↑][864]** [Klabnik & Nichols 2023][865],

    p. 32.

50. **[↑][866]** [Klabnik & Nichols 2023][865],

    pp. 32–33.

51. **[↑][867]** [Klabnik & Nichols 2023][865],

    pp. 49–50.

52. **[↑][868]** [Klabnik & Nichols 2023][865],

    pp. 34–36.

53. **[↑][869]** [Klabnik & Nichols 2023][865],

    pp. 6, 47, 53.

54. **[↑][870]** [Klabnik & Nichols 2023][865],

    pp. 47–48.

55. ^ [***a***][871] [***b***][872] [Klabnik & Nichols 2023][865],

    pp. 50–53.

56. **[↑][873]** [Klabnik & Nichols 2023][865],

    p. 56.

57. **[↑][874]** [Klabnik & Nichols 2023][865],

    pp. 57–58.

58. **[↑][875]** [Klabnik & Nichols 2023][865],

    pp. 54–56.

59. **[↑][876]** [Klabnik & Nichols 2019][862],

    pp. 104–109.

60. **[↑][877]** [Klabnik & Nichols 2019][862],

    pp. 24.

61. **[↑][878]** [Klabnik & Nichols 2019][862],

    pp. 36–38.

62. **[↑][879]** ["isize"][880]. *doc.rust-lang.org*. Retrieved 2025-09-28.
63. **[↑][881]** ["usize"][882]. *doc.rust-lang.org*. Retrieved 2025-09-28.
64. **[↑][883]** [Klabnik & Nichols 2023][865],

    pp. 36–38.

65. **[↑][884]** [Klabnik & Nichols 2023][865],

    p. 502.

66. **[↑][885]** ["Primitive Type char"][886]. *The Rust Standard Library documentation*. Retrieved 2025-09-07.
67. **[↑][887]** ["Glossary of Unicode Terms"][888]. *[Unicode Consortium][889]*. [Archived][890] from the original on 2018-09-24. Retrieved 2024-07-30.
68. **[↑][891]** [Klabnik & Nichols 2019][862],

    pp. 38–40.

69. **[↑][892]** [Klabnik & Nichols 2023][865],

    pp. 40–42.

70. **[↑][893]** [Klabnik & Nichols 2023][865],

    p. 42.

71. **[↑][894]** [Klabnik & Nichols 2019][862],

    pp. 59–61.

72. ^ [***a***][895] [***b***][896] [Klabnik & Nichols 2019][862],

    pp. 63–68.

73. **[↑][897]** [Klabnik & Nichols 2019][862],

    pp. 74–75.

74. **[↑][898]** [Klabnik & Nichols 2023][865],

    pp. 71–72.

75. ^ [***a***][899] [***b***][900] Balasubramanian, Abhiram; Baranowski, Marek S.; Burtsev, Anton;

    Panda, Aurojit; Rakamarić, Zvonimir; Ryzhyk, Leonid (2017-05-07). ["System Programming in Rust"][901]. *Proceedings of the 16th Workshop on Hot Topics in Operating
    Systems*. HotOS '17. New York, NY, US: Association for Computing Machinery.
    pp. 156–161. [doi][685]:[10.1145/3102980.3103006][902]. [ISBN][659] [978-1-4503-5068-6][903]. [S2CID][688] [24100599][904]. [Archived][905] from the original on 2022-06-11. Retrieved 2022-06-01.

76. **[↑][906]** [Klabnik & Nichols 2023][865],

    pp. 327–30.

77. **[↑][907]** ["Lifetimes"][908]. *Rust by Example*. [Archived][909] from the original on 2024-11-16. Retrieved 2024-10-29.
78. **[↑][910]** ["Explicit annotation"][911]. *Rust by Example*. Retrieved 2024-10-29.
79. ^ [***a***][912] [***b***][913] [Klabnik & Nichols 2019][862],

    p. 194.

80. **[↑][914]** [Klabnik & Nichols 2019][862],

    pp. 75, 134.

81. **[↑][915]** Shamrell-Harrington, Nell (2022-04-15). ["The Rust Borrow Checker – a Deep Dive"][916]. *InfoQ*. [Archived][917] from the original on 2022-06-25. Retrieved 2022-06-25.
82. **[↑][918]** [Klabnik & Nichols 2019][862],

    pp. 194–195.

83. **[↑][919]** [Klabnik & Nichols 2023][865],

    pp. 208–12.

84. **[↑][920]** [Klabnik & Nichols 2023][865], [4.2. References and Borrowing][921].
85. **[↑][922]** Pearce, David (2021-04-17). ["A Lightweight Formalism for Reference Lifetimes and Borrowing in Rust"][923]. *ACM Transactions on Programming Languages and Systems*. **43**: 1–73. [doi][685]:[10.1145/3443420][924]. [Archived][925] from the original on 2024-04-15. Retrieved 2024-12-11.
86. **[↑][926]** [Klabnik & Nichols 2019][862],

    p. 83.

87. **[↑][927]** [Klabnik & Nichols 2019][862],

    p. 97.

88. **[↑][928]** [Klabnik & Nichols 2019][862],

    pp. 98–101.

89. **[↑][929]** [Klabnik & Nichols 2019][862],

    pp. 438–440.

90. **[↑][930]** [Klabnik & Nichols 2019][862],

    pp. 93.

91. **[↑][931]** [Gjengset 2021][932],

    pp. 213–215.

92. **[↑][933]** [Klabnik & Nichols 2023][865],

    pp. 108–110, 113–114, 116–117.

93. **[↑][934]** ["Rust error handling is perfect actually"][935]. *Bitfield Consulting*. [Archived][936] from the original on 2025-08-07. Retrieved 2025-09-15.
94. **[↑][937]** [Gjengset 2021][932],

    p. 155-156.

95. **[↑][938]** [Klabnik & Nichols 2023][865],

    pp. 421–423.

96. **[↑][939]** [Klabnik & Nichols 2019][862],

    pp. 418–427.

97. **[↑][940]** ["Casting"][941]. *Rust by Example*. Retrieved 2025-04-01.
98. **[↑][942]** [Klabnik & Nichols 2023][865],

    p. 378.

99. ^ [***a***][943] [***b***][944] [***c***][945] [Klabnik & Nichols 2023][865],

    pp. 192–198.

100. **[↑][946]** [Klabnik & Nichols 2023][865],

     p. 98.

101. **[↑][947]** [Klabnik & Nichols 2019][862],

     pp. 171–172, 205.

102. ^ [***a***][948] [***b***][949] [Klabnik & Nichols 2023][865],

     pp. 191–192.

103. **[↑][950]** [Gjengset 2021][932], p. 25.
104. **[↑][951]** [Klabnik & Nichols 2023][865], [18.2. Using Trait Objects That Allow for Values of Different Types][952].
105. **[↑][953]** [Klabnik & Nichols 2019][862],

     pp. 441–442.

106. **[↑][954]** [Klabnik & Nichols 2019][862],

     pp. 379–380.

107. **[↑][955]** Rosenblatt, Seth (2013-04-03). ["Samsung joins Mozilla's quest for Rust"][956]. [CNET][957]. [Archived][958] from the original on 2013-04-04. Retrieved 2013-04-05.
108. **[↑][959]** Brown, Neil (2013-04-17). ["A taste of Rust"][960]. *[LWN.net][961]*. [Archived][962] from the original on 2013-04-26. Retrieved 2013-04-25.
109. **[↑][963]** ["Races"][964]. *The Rustonomicon*. [Archived][965] from the original on 2017-07-10. Retrieved 2017-07-03.
110. **[↑][966]** Vandervelden, Thibaut; De Smet, Ruben; Deac, Diana; Steenhaut, Kris;

     Braeken, An (2024-09-07). ["Overview of Embedded Rust Operating Systems and Frameworks"][967]. *Sensors*. **24** (17): 5818. [Bibcode][968]:[2024Senso..24.5818V][969]. [doi][685]:[10.3390/s24175818][970]. [PMC][971] [11398098][967]. [PMID][972] [39275729][973].

111. **[↑][974]** ["The Rust Language FAQ"][975]. The Rust Programming Language. 2015. Archived from [the original][976] on 2015-04-20. Retrieved 2017-04-24.
112. **[↑][977]** ["RAII"][978]. *Rust by Example*. [Archived][979] from the original on 2019-04-21. Retrieved 2020-11-22.
113. **[↑][980]** ["Abstraction without overhead: traits in Rust"][981]. *Rust Blog*. [Archived][982] from the original on 2021-09-23. Retrieved 2021-10-19.
114. **[↑][983]** ["Box, stack and heap"][984]. *Rust by Example*. [Archived][985] from the original on 2022-05-31. Retrieved 2022-06-13.
115. **[↑][986]** [Klabnik & Nichols 2019][862],

     pp. 70–75.

116. **[↑][987]** [Klabnik & Nichols 2019][862],

     p. 323.

117. **[↑][988]** [Klabnik & Nichols 2023][865],

     pp. 420–429.

118. **[↑][989]** [McNamara 2021][990], p. 139, 376–379,
119.
120. ^ [***a***][991] [***b***][992] Astrauskas, Vytautas; Matheja, Christoph; Poli, Federico; Müller,

     Peter; Summers, Alexander J. (2020-11-13). ["How do programmers use unsafe rust?"][993]. *Proceedings of the ACM on Programming Languages*. **4**: 1–27. [doi][685]:[10.1145/3428204][994]. [hdl][805]:[20.500.11850/465785][995]. [ISSN][996] [2475-1421][997].

121. **[↑][998]** Lattuada, Andrea; Hance, Travis; Cho, Chanhee; Brun, Matthias;

     Subasinghe, Isitha; Zhou, Yi; Howell, Jon; Parno, Bryan; Hawblitzel,
     Chris (2023-04-06). ["Verus: Verifying Rust Programs using Linear Ghost Types"][999]. *Proceedings of the ACM on Programming Languages*. **7**:
     85:286–85:315. [doi][685]:[10.1145/3586037][1000]. [hdl][805]:[20.500.11850/610518][1001].

122. **[↑][1002]** Milano, Mae; Turcotti, Julia; Myers, Andrew C. (2022-06-09). "A

     flexible type system for fearless concurrency". *Proceedings of the 43rd ACM SIGPLAN International Conference on
     Programming Language Design and Implementation*. New York, NY, USA: Association for Computing Machinery.
     pp. 458–473. [doi][685]:[10.1145/3519939.3523443][1003]. [ISBN][659] [978-1-4503-9265-5][1004].

123. **[↑][1005]** ["Introduction - Learning Rust With Entirely Too Many Linked Lists"][1006]. *rust-unofficial.github.io*. Retrieved 2025-08-06.
124. **[↑][1007]** Noble, James; Mackay, Julian; Wrigstad, Tobias (2023-10-16). ["Rusty Links in Local Chains✱"][1008]. *Proceedings of the 24th ACM International Workshop on Formal

     Techniques for Java-like Programs*. New York, NY, USA: Association for Computing Machinery.
     pp. 1–3. [doi][685]:[10.1145/3611096.3611097][1009]. [ISBN][659] [979-8-4007-0784-1][1010].

125. ^ [***a***][1011] [***b***][1012] Evans, Ana Nora; Campbell, Bradford; Soffa, Mary Lou (2020-10-01). ["Is rust used safely by software developers?"][1013]. *Proceedings of the ACM/IEEE 42nd International Conference on

     Software Engineering*. New York, NY, USA: Association for Computing Machinery.
     pp. 246–257. [arXiv][708]:[2007.00752][1014]. [doi][685]:[10.1145/3377811.3380413][1015]. [ISBN][659] [978-1-4503-7121-6][1016].

126. **[↑][1017]** ["Behavior considered undefined"][1018]. *The Rust Reference*. Retrieved 2025-08-06.
127. **[↑][1019]** [Klabnik & Nichols 2023][865],

     pp. 449–455.

128. **[↑][1020]** [Gjengset 2021][932],

     pp. 101–102.

129. **[↑][1021]** ["Macros By Example"][1022]. *The Rust Reference*. [Archived][1023] from the original on 2023-04-21. Retrieved 2023-04-21.
130. **[↑][1024]** [Klabnik & Nichols 2019][862],

     pp. 446–448.

131. **[↑][1025]** ["Procedural Macros"][1026]. *The Rust Programming Language Reference*. [Archived][1027] from the original on 2020-11-07. Retrieved 2021-03-23.
132. **[↑][1028]** [Klabnik & Nichols 2019][862],

     pp. 449–455.

133. **[↑][1029]** Baumgartner, Stefan (2025-05-23). ["Programming language: Rust 2024 is the most comprehensive edition to date"][1030]. *[heise online][1031]*. Retrieved 2025-06-28.
134. ^ [***a***][1032] [***b***][1033] [***c***][1034] [Gjengset 2021][932],

     pp. 193–209.

135. **[↑][1035]** ["Safe Interoperability between Rust and C++ with CXX"][1036]. *InfoQ*. 2020-12-06. [Archived][1037] from the original on 2021-01-22. Retrieved 2021-01-03.
136. **[↑][1038]** [Blandy, Orendorff & Tindall 2021][1039], pp. 6–8.
137. **[↑][1040]** ["Overview of the compiler"][1041]. *Rust Compiler Development Guide*. Rust project contributors. [Archived][1042] from the original on 2023-05-31. Retrieved 2024-11-07.
138. **[↑][1043]** ["Code Generation"][1044]. *Rust Compiler Development Guide*. Rust project

     contributors. Retrieved 2024-03-03.

139. **[↑][1045]** ["rust-lang/rustc_codegen_gcc"][1046]. *[GitHub][592]*. The Rust Programming Language. 2024-03-02. Retrieved 2024-03-03.
140. **[↑][1047]** ["rust-lang/rustc_codegen_cranelift"][1048]. *[GitHub][592]*. The Rust Programming Language. 2024-03-02. Retrieved 2024-03-03.
141. ^ [***a***][1049] [***b***][1050] [***c***][1051] Perkel, Jeffrey M. (2020-12-01). ["Why scientists are turning to Rust"][1052]. *[Nature][581]*. **588** (7836): 185–186. [Bibcode][968]:[2020Natur.588..185P][1053]. [doi][685]:[10.1038/d41586-020-03382-2][1054]. [PMID][972] [33262490][1055]. [S2CID][688] [227251258][1056]. [Archived][1057] from the original on 2022-05-06. Retrieved 2022-05-15.
142. **[↑][1058]** Simone, Sergio De (2019-04-18). ["Rust 1.34 Introduces Alternative Registries for Non-Public Crates"][1059]. *InfoQ*. [Archived][1060] from the original on 2022-07-14. Retrieved 2022-07-14.
143. **[↑][1061]** [Klabnik & Nichols 2019][862],

     pp. 511–512.

144. **[↑][1062]** ["Clippy"][1063]. *[GitHub][592]*. The Rust Programming Language. 2023-11-30. [Archived][1064] from the original on 2021-05-23. Retrieved 2023-11-30.
145. **[↑][1065]** ["Clippy Lints"][1066]. *The Rust Programming Language*. Retrieved 2023-11-30.
146. **[↑][1067]** [Klabnik & Nichols 2019][862],

     Appendix G – How Rust is Made and "Nightly Rust"

147. **[↑][1068]** [Blandy, Orendorff & Tindall 2021][1039], pp. 176–177.
148. **[↑][1069]** [Klabnik & Nichols 2023][865],

     p. 623.

149. **[↑][1070]** Anderson, Tim (2021-11-30). ["Can Rust save the planet? Why, and why not"][1071]. *[The Register][832]*. [Archived][1072] from the original on 2022-07-11. Retrieved 2022-07-11.
150. **[↑][1073]** Yegulalp, Serdar (2021-10-06). ["What is the Rust language? Safe, fast, and easy software development"][1074]. *[InfoWorld][718]*. [Archived][1075] from the original on 2022-06-24. Retrieved 2022-06-25.
151. **[↑][1076]** [McNamara 2021][990], p. 11.
152. ^ [***a***][1077] [***b***][1078] Popescu, Natalie; Xu, Ziyang; Apostolakis, Sotiris; August, David I.;

     Levy, Amit (2021-10-15). ["Safer at any speed: automatic context-aware safety enhancement for Rust"][1079]. *Proceedings of the ACM on Programming Languages*. **5** (OOPSLA). Section 2. [doi][685]:[10.1145/3485480][1079]. [S2CID][688] [238212612][1080]. p. 5: "We observe a large variance in the overheads of checked indexing:
     23.6% of benchmarks do report significant performance hits from
     checked indexing, but 64.5% report little-to-no impact and,
     surprisingly, 11.8% report improved performance ... Ultimately,
     while unchecked indexing can improve performance, most of the time
     it does not."

153. **[↑][1081]** [McNamara 2021][990], p. 19, 27.
154. **[↑][1082]** Couprie, Geoffroy (2015). "Nom, A Byte oriented, streaming, Zero

     copy, Parser Combinators Library in Rust". *2015 IEEE Security and Privacy Workshops*. pp. 142–148. [doi][685]:[10.1109/SPW.2015.31][1083]. [ISBN][659] [978-1-4799-9933-0][1084]. [S2CID][688] [16608844][1085].

155. **[↑][1086]** [McNamara 2021][990], p. 20.
156. **[↑][1087]** ["Code generation"][1088]. *The Rust Reference*. [Archived][1089] from the original on 2022-10-09. Retrieved 2022-10-09.
157. **[↑][1090]** ["How Fast Is Rust?"][1091]. *The Rust Programming Language FAQ*. [Archived][1092] from the original on 2020-10-28. Retrieved 2019-04-11.
158. **[↑][1093]** Farshin, Alireza; Barbette, Tom; Roozbeh, Amir; Maguire, Gerald Q.

     Jr; Kostić, Dejan (2021). "PacketMill: Toward per-Core 100-GBPS
     networking". [*Proceedings of the 26th ACM International Conference on Architectural Support for Programming Languages and Operating Systems*][1094]. pp. 1–17. [doi][685]:[10.1145/3445814.3446724][1095]. [ISBN][659] [9781450383172][1096]. [S2CID][688] [231949599][1097]. [Archived][1098] from the original on 2022-07-12. Retrieved 2022-07-12. "... While some compilers (e.g., Rust) support structure reordering
     [82], C & C++ compilers are forbidden to reorder data structures
     (e.g., struct or class) [74] ..."

159. **[↑][1099]** [Gjengset 2021][932], p. 22.
160. **[↑][1100]** Keizer, Gregg (2016-10-31). ["Mozilla plans to rejuvenate Firefox in 2017"][1101]. *[Computerworld][1102]*. [Archived][1103] from the original on 2023-05-13. Retrieved 2023-05-13.
161. **[↑][1104]** Claburn, Thomas (2023-01-12). ["Google polishes Chromium code with a layer of Rust"][1105]. *[The Register][832]*. Retrieved 2024-07-17.
162. **[↑][1106]** Jansens, Dana (2023-01-12). ["Supporting the Use of Rust in the Chromium Project"][1107]. *Google Online Security Blog*. [Archived][1108] from the original on 2023-01-13. Retrieved 2023-11-12.
163. **[↑][1109]** Shankland, Stephen (2016-07-12). ["Firefox will get overhaul in bid to get you interested again"][1110]. [CNET][957]. [Archived][1111] from the original on 2022-07-14. Retrieved 2022-07-14.
164. **[↑][1112]** Security Research Team (2013-10-04). ["ZeroMQ: Helping us Block Malicious Domains in Real Time"][1113]. *Cisco Umbrella*. Archived from [the original][1114] on 2023-05-13. Retrieved 2023-05-13.
165. **[↑][1115]** Cimpanu, Catalin (2019-10-15). ["AWS to sponsor Rust project"][1116]. *[ZDNET][705]*. Retrieved 2024-07-17.
166. **[↑][1117]** Nichols, Shaun (2018-06-27). ["Microsoft's next trick? Kicking things out of the cloud to Azure IoT Edge"][1118]. *[The Register][832]*. [Archived][1119] from the original on 2019-09-27. Retrieved 2019-09-27.
167. **[↑][1120]** Tongue, Liam (2020-04-30). ["Microsoft: Why we used programming language Rust over Go for WebAssembly on Kubernetes app"][1121]. *[ZDNET][705]*. [Archived][1122] from the original on 2022-04-21. Retrieved 2022-04-21.
168. **[↑][1123]** Claburn, Thomas (2022-09-20). ["In Rust We Trust: Microsoft Azure CTO shuns C and C++"][1124]. *[The Register][832]*. Retrieved 2024-07-07.
169. **[↑][1125]** Simone, Sergio De (2019-03-10). ["NPM Adopted Rust to Remove Performance Bottlenecks"][1126]. *InfoQ*. [Archived][1127] from the original on 2023-11-19. Retrieved 2023-11-20.
170. **[↑][1128]** Lyu, Shing (2020). ["Welcome to the World of Rust"][1129]. In Lyu, Shing (ed.). *Practical Rust Projects: Building Game, Physical Computing, and

     Machine Learning Applications*. Berkeley, CA: Apress. pp. 1–8. [doi][685]:[10.1007/978-1-4842-5599-5_1][1130]. [ISBN][659] [978-1-4842-5599-5][1131]. Retrieved 2023-11-29.

171. **[↑][1132]** Lyu, Shing (2021). ["Rust in the Web World"][1133]. In Lyu, Shing (ed.). *Practical Rust Web Projects: Building Cloud and Web-Based

     Applications*. Berkeley, CA: Apress. pp. 1–7. [doi][685]:[10.1007/978-1-4842-6589-5_1][1134]. [ISBN][659] [978-1-4842-6589-5][1135]. Retrieved 2023-11-29.

172. ^ [***a***][1136] [***b***][1137] [***c***][1138] Li, Hongyu; Guo, Liwei; Yang, Yexuan; Wang, Shangguang; Xu, Mengwei

     (2024-06-30). ["An Empirical Study of Rust-for-Linux: The Success, Dissatisfaction, and Compromise"][1139]. *[USENIX][1140]*. Retrieved 2024-11-28.

173. **[↑][1141]** Corbet, Jonathan (2022-10-13). ["A first look at Rust in the 6.1 kernel"][1142]. *[LWN.net][961]*. [Archived][1143] from the original on 2023-11-17. Retrieved 2023-11-11.
174. **[↑][1144]** Vaughan-Nichols, Steven (2021-12-07). ["Rust takes a major step forward as Linux's second official language"][1145]. *[ZDNET][705]*. Retrieved 2024-11-27.
175. **[↑][1146]** Corbet, Jonathan (2022-11-17). ["Rust in the 6.2 kernel"][1147]. *[LWN.net][961]*. Retrieved 2024-11-28.
176. **[↑][1148]** Corbet, Jonathan (2024-09-24). ["Committing to Rust in the kernel"][1149]. *[LWN.net][961]*. Retrieved 2024-11-28.
177. **[↑][1150]** Amadeo, Ron (2021-04-07). ["Google is now writing low-level Android code in Rust"][828]. *Ars Technica*. [Archived][829] from the original on 2021-04-08. Retrieved 2022-04-21.
178. **[↑][1151]** Darkcrizt (2021-04-02). ["Google Develops New Bluetooth Stack for Android, Written in Rust"][1152]. *Desde Linux*. Archived from [the original][1153] on 2021-08-25. Retrieved 2024-08-31.
179. **[↑][1154]** Claburn, Thomas (2023-04-27). ["Microsoft is rewriting core Windows libraries in Rust"][1155]. *[The Register][832]*. [Archived][1156] from the original on 2023-05-13. Retrieved 2023-05-13.
180. **[↑][1157]** Proven, Liam (2023-12-01). ["Small but mighty, 9Front's 'Humanbiologics' is here for the truly curious"][1158]. *[The Register][832]*. Retrieved 2024-03-07.
181. **[↑][1159]** Yegulalp, Serdar (2016-03-21). ["Rust's Redox OS could show Linux a few new tricks"][1160]. *[InfoWorld][718]*. Archived from [the original][1161] on 2016-03-21. Retrieved 2016-03-21.
182. **[↑][1162]** Anderson, Tim (2021-01-14). ["Another Rust-y OS: Theseus joins Redox in pursuit of safer, more resilient systems"][1163]. *[The Register][832]*. [Archived][1164] from the original on 2022-07-14. Retrieved 2022-07-14.
183. **[↑][1165]** Boos, Kevin; Liyanage, Namitha; Ijaz, Ramla; Zhong, Lin (2020). [*Theseus: an Experiment in Operating System Structure and State Management*][1166]. pp. 1–19. [ISBN][659] [978-1-939133-19-9][1167]. [Archived][1168] from the original on 2023-05-13. Retrieved 2023-05-13.
184. **[↑][1169]** Zhang, HanDong (2023-01-31). ["2022 Review | The adoption of Rust in Business"][1170]. *Rust Magazine*. Retrieved 2023-02-07.
185. **[↑][1171]** Sei, Mark (2018-10-10). ["Fedora 29 new features: Startis now officially in Fedora"][1172]. *Marksei, Weekly sysadmin pills*. [Archived][1173] from the original on 2019-04-13. Retrieved 2019-05-13.
186. **[↑][1174]** Proven, Liam (2022-07-12). ["Oracle Linux 9 released, with some interesting additions"][1175]. *[The Register][832]*. [Archived][1176] from the original on 2022-07-14. Retrieved 2022-07-14.
187. **[↑][1177]** Proven, Liam (2023-02-02). ["System76 teases features coming in homegrown Rust-based desktop COSMIC"][1178]. *[The Register][832]*. [Archived][1179] from the original on 2024-07-17. Retrieved 2024-07-17.
188. **[↑][1180]** Hu, Vivian (2020-06-12). ["Deno Is Ready for Production"][1181]. *InfoQ*. [Archived][1182] from the original on 2020-07-01. Retrieved 2022-07-14.
189. **[↑][1183]** Abrams, Lawrence (2021-02-06). ["This Flash Player emulator lets you securely play your old games"][1184]. *[Bleeping Computer][1185]*. [Archived][1186] from the original on 2021-12-25. Retrieved 2021-12-25.
190. **[↑][1187]** Kharif, Olga (2020-10-17). ["Ethereum Blockchain Killer Goes By Unassuming Name of Polkadot"][1188]. *[Bloomberg News][1189]*. [Bloomberg L.P.][1190] [Archived][1191] from the original on 2020-10-17. Retrieved 2021-07-14.
191. ^ [***a***][1192] [***b***][1193] [***c***][1194] ["2025 Stack Overflow Developer Survey – Technology"][1195]. *[Stack Overflow][569]*. Retrieved 2025-08-09.
192. **[↑][1196]** Claburn, Thomas (2022-06-23). ["Linus Torvalds says Rust is coming to the Linux kernel"][1197]. *[The Register][832]*. [Archived][1198] from the original on 2022-07-28. Retrieved 2022-07-15.
193. **[↑][1199]** Wallach, Dan. ["TRACTOR: Translating All C to Rust"][1200]. [DARPA][573]. Retrieved 2025-08-03.
194. **[↑][1201]** Jung, Ralf; Jourdan, Jacques-Henri; Krebbers, Robbert; Dreyer, Derek

     (2017-12-27). ["RustBelt: securing the foundations of the Rust programming language"][1202]. *Proceedings of the ACM on Programming Languages*. **2** (POPL): 1–34. [doi][685]:[10.1145/3158154][1203]. [hdl][805]:[21.11116/0000-0003-34C6-3][1204]. [ISSN][996] [2475-1421][997].

195. **[↑][1205]** Popescu, Natalie; Xu, Ziyang; Apostolakis, Sotiris; August, David I.;

     Levy, Amit (2021-10-20). ["Safer at any speed: automatic context-aware safety enhancement for Rust"][1079]. *Proceedings of the ACM on Programming Languages*. **5** (OOPSLA): 1–23. [doi][685]:[10.1145/3485480][1079]. [ISSN][996] [2475-1421][997].

196. **[↑][1206]** Blanco-Cuaresma, Sergi; Bolmont, Emeline (2017-05-30). ["What can the programming language Rust do for astrophysics?"][1207]. *[Proceedings of the International Astronomical Union][579]*. **12** (S325): 341–344. [arXiv][708]:[1702.02951][1208]. [Bibcode][968]:[2017IAUS..325..341B][1209]. [doi][685]:[10.1017/S1743921316013168][1210]. [ISSN][996] [1743-9213][1211]. [S2CID][688] [7857871][1212]. [Archived][1213] from the original on 2022-06-25. Retrieved 2022-06-25.
197. **[↑][1214]** [Klabnik & Nichols 2019][862],

     p. 4.

198. **[↑][1215]** ["Getting Started"][1216]. *The Rust Programming Language*. [Archived][1217] from the original on 2020-11-01. Retrieved 2020-10-11.
199. **[↑][1218]** The Rust Survey Team (2025-02-13). ["2024 State of Rust Survey Results"][1219]. *The Rust Programming Language*. Retrieved 2025-09-07.
200. **[↑][1220]** ["The top programming languages"][1221]. *The State of the Octoverse*. 2022. Retrieved 2025-06-25.
201. **[↑][1222]** ["Octoverse: AI leads Python to top language as the number of global developers surges"][1223]. *The GitHub Blog*. 2024-10-29. Retrieved 2025-06-25.
202. **[↑][1224]** Tongue, Liam (2021-02-08). ["The Rust programming language just took a huge step forwards"][1225]. *[ZDNET][705]*. [Archived][1226] from the original on 2022-07-14. Retrieved 2022-07-14.
203. **[↑][1227]** Krill, Paul (2021-02-09). ["Rust language moves to independent foundation"][1228]. *[InfoWorld][718]*. [Archived][1229] from the original on 2021-04-10. Retrieved 2021-04-10.
204. **[↑][1230]** Vaughan-Nichols, Steven J. (2021-04-09). ["AWS's Shane Miller to head the newly created Rust Foundation"][1231]. *[ZDNET][705]*. [Archived][1232] from the original on 2021-04-10. Retrieved 2021-04-10.
205. **[↑][1233]** Vaughan-Nichols, Steven J. (2021-11-17). ["Rust Foundation appoints Rebecca Rumbul as executive director"][1234]. *[ZDNET][705]*. [Archived][1235] from the original on 2021-11-18. Retrieved 2021-11-18.
206. **[↑][1236]** ["Governance"][1237]. *The Rust Programming Language*. [Archived][1238] from the original on 2022-05-10. Retrieved 2024-07-18.
207. **[↑][1239]** ["Introducing the Rust Leadership Council"][1240]. *Rust Blog*. 2023-06-20. Retrieved 2024-01-12.

### External links

[[edit][1241]]

**Rust** at Wikipedia's [sister projects][1242]

- [/wiki/File:Commons-logo.svg][1244][Media][1245] from Commons
- [/wiki/File:Wikiversity_logo_2017.svg][1247][Resources][1248] from Wikiversity
- [/wiki/File:Wikidata-logo.svg][1250][Data][1251] from Wikidata

- [Official website][86] [https://www.wikidata.org/wiki/Q575650#P856][1252]
- [Source code][1253] on [GitHub][592]
- [Documentation][1254]


| [v (View this template)][1255] [t (Discuss this template)][1256] [e (Edit this template)][1257]   [Programming languages][122]                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --- |
| [Comparison][617] [Timeline][1258] [History][618]                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | |
| [Ada][1259] [ALGOL][1260]<br>  [Simula][1261] [APL][1262] [Assembly][143] [BASIC][1263]<br>  [Visual Basic][1264]<br>    [classic][1265]    [.NET][1266] [C][142][C++][91] [C#][90][COBOL][1267] [Erlang][94]<br>  [Elixir][1268] [Forth][1269][Fortran][1270] [Go][1271][Haskell][95] [Java][199] [JavaScript][555] [Julia][1272] [Kotlin][1273] [Lisp][1274][Lua][1275][MATLAB][1276] [ML][1277]<br>  [Caml][1278]<br>    [OCaml][75] [Pascal][1279]<br>  [Object Pascal][1280] [Perl][1281]<br>  [Raku][1282] [PHP][1283][Prolog][1284] [Python][1285] [R][1286] [Ruby][103]Rust[SAS][1287][SQL][1288] [Scratch][1289][Shell][1290][Smalltalk][1291] [Swift][107] *[more...][619]* | |
| **Lists:** [Alphabetical][619] [Categorical][620] [Generational][1293] [Non-English-based][1294] [Category][1296]                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | |

| [v (View this template)][1297] [t (Discuss this template)][1298] [e (Edit this template)][1299]   [Mozilla][138]                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | |
| --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --- |
|  | Projects |
|  | Organization |
|  | Community |
|  | Other topics |

| [Authority control databases][1399] [https://www.wikidata.org/wiki/Q575650#identifiers][1400] | |
| --------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| International                                                                                 | [GND][1401] [FAST][1402]                                                                                  |
| National                                                                                      | [United States][1403] [France][1404] [BnF data][1405] [Czech Republic][1406] [Spain][1407] [Israel][1408] |
| Other                                                                                         | [Yale LUX][1409]                                                                                          |

[Portal][1410]:

- [/wiki/File:Octicons-terminal.svg][1412] [Computer programming][1413]

Retrieved from "<https://en.wikipedia.org/w/index.php?title=Rust_(programming_language)&oldid=1314392250>"

[Categories][1415]:

- [Rust (programming language)][1416]
- [Compiled programming languages][1417]
- [Concurrent programming languages][1418]
- [Free and open source compilers][1419]
- [Free software projects][1420]
- [Functional languages][1421]
- [High-level programming languages][1422]
- [Mozilla][1423]
- [Multi-paradigm programming languages][1424]
- [Pattern matching programming languages][1425]
- [Procedural programming languages][1426]
- [Programming languages created in 2015][1427]
- [Software using the Apache license][1428]
- [Software using the MIT license][1429]
- [Statically typed programming languages][1430]
- [Systems programming languages][1431]

Hidden categories:

- [Articles with short description][1432]
- [Short description is different from Wikidata][1433]
- [Good articles][1434]
- [Use American English from July 2022][1435]
- [All Wikipedia articles written in American English][1436]
- [Use mdy dates from July 2022][1437]
- [Articles with example C++ code][1438]
- [Articles with excerpts][1439]
- [Articles containing potentially dated statements from 2024][1440]
- [All articles containing potentially dated statements][1441]
- [Articles containing potentially dated statements from July 2024][1442]
-
- [Articles with example Rust code][1443]

- This page was last edited on 1 October 2025, at 04:26 (UTC).
- Text is available under the [Creative Commons Attribution-ShareAlike 4.0 License][1444]; additional terms may apply. By using this site, you agree to the [Terms of Use][1445] and [Privacy Policy][1446]. Wikipedia® is a registered trademark of the [Wikimedia Foundation, Inc.][1447], a non-profit organization.


- [Privacy policy][1448]
- [About Wikipedia][1449]
- [Disclaimers][1450]
- [Contact Wikipedia][1451]
- [Code of Conduct][1452]
- [Developers][1453]
- [Statistics][1454]
- [Cookie statement][1455]
- [Mobile view][1456]


- [https://www.wikimedia.org/][1458]
- [https://www.mediawiki.org/][1460]

Search

Rust (programming language)

[#][1461] [#][1461] [#][1461] [#][1461] [#][1461] [#][1461] [#][1461] [#][1461]

51 languages

[Add topic][1461]

[1]: #bodyContent
[2]: https://af.wikipedia.org/wiki/Rust_(programmeertaal) "Rust (programmeertaal) – Afrikaans"
[3]: https://ar.wikipedia.org/wiki/%D8%B1%D8%B3%D8%AA_(%D9%84%D8%BA%D8%A9_%D8%A8%D8%B1%D9%85%D8%AC%D8%A9) "رست (لغة برمجة) – Arabic"
[4]: https://az.wikipedia.org/wiki/Rust_(proqramla%C5%9Fd%C4%B1rma_dili) "Rust (proqramlaşdırma dili) – Azerbaijani"
[5]: https://bn.wikipedia.org/wiki/%E0%A6%B0%E0%A6%BE%E0%A6%B8%E0%A7%8D%E0%A6%9F_(%E0%A6%AA%E0%A7%8D%E0%A6%B0%E0%A7%8B%E0%A6%97%E0%A7%8D%E0%A6%B0%E0%A6%BE%E0%A6%AE%E0%A6%BF%E0%A6%82_%E0%A6%AD%E0%A6%BE%E0%A6%B7%E0%A6%BE) "রাস্ট (প্রোগ্রামিং ভাষা) – Bangla"
[6]: https://zh-min-nan.wikipedia.org/wiki/Rust_(pian-t%C3%AAng_g%C3%BA-gi%C3%A2n) "Rust (pian-têng gú-giân) – Minnan"
[7]: https://be.wikipedia.org/wiki/Rust "Rust – Belarusian"
[8]: https://ca.wikipedia.org/wiki/Rust_(llenguatge_de_programaci%C3%B3) "Rust (llenguatge de programació) – Catalan"
[9]: https://cs.wikipedia.org/wiki/Rust_(programovac%C3%AD_jazyk) "Rust (programovací jazyk) – Czech"
[10]: https://da.wikipedia.org/wiki/Rust_(programmeringssprog) "Rust (programmeringssprog) – Danish"
[11]: https://de.wikipedia.org/wiki/Rust_(Programmiersprache) "Rust (Programmiersprache) – German"
[12]: https://et.wikipedia.org/wiki/Rust "Rust – Estonian"
[13]: https://es.wikipedia.org/wiki/Rust_(lenguaje_de_programaci%C3%B3n) "Rust (lenguaje de programación) – Spanish"
[14]: https://eo.wikipedia.org/wiki/Rust_(programlingvo) "Rust (programlingvo) – Esperanto"
[15]: https://eu.wikipedia.org/wiki/Rust_(programazio-lengoaia) "Rust (programazio-lengoaia) – Basque"
[16]: https://fa.wikipedia.org/wiki/%D8%B1%D8%A7%D8%B3%D8%AA_(%D8%B2%D8%A8%D8%A7%D9%86_%D8%A8%D8%B1%D9%86%D8%A7%D9%85%D9%87%E2%80%8C%D9%86%D9%88%DB%8C%D8%B3%DB%8C) "راست (زبان برنامه‌نویسی) – Persian"
[17]: https://fr.wikipedia.org/wiki/Rust_(langage) "Rust (language) – French"
[18]: https://gl.wikipedia.org/wiki/Rust_(linguaxe_de_programaci%C3%B3n) "Rust (linguaxe de programación) – Galician"
[19]: https://ko.wikipedia.org/wiki/%EB%9F%AC%EC%8A%A4%ED%8A%B8_(%ED%94%84%EB%A1%9C%EA%B7%B8%EB%9E%98%EB%B0%8D_%EC%96%B8%EC%96%B4) "러스트 (프로그래밍 언어) – Korean"
[20]: https://hr.wikipedia.org/wiki/Rust_(programski_jezik) "Rust (programski jezik) – Croatian"
[21]: https://id.wikipedia.org/wiki/Rust_(bahasa_pemrograman) "Rust (bahasa pemrograman) – Indonesian"
[22]: https://is.wikipedia.org/wiki/Rust_(forritunarm%C3%A1l) "Rust (forritunarmál) – Icelandic"
[23]: https://it.wikipedia.org/wiki/Rust_(linguaggio_di_programmazione) "Rust (linguaggio di programmazione) – Italian"
[24]: https://he.wikipedia.org/wiki/%D7%A8%D7%90%D7%A1%D7%98_(%D7%A9%D7%A4%D7%AA_%D7%AA%D7%9B%D7%A0%D7%95%D7%AA) "ראסט (שפת תכנות) – Hebrew"
[25]: https://sw.wikipedia.org/wiki/Rust_(lugha_ya_kompyuta) "Rust (lugha ya kompyuta) – Swahili"
[26]: https://lv.wikipedia.org/wiki/Rust "Rust – Latvian"
[27]: https://lmo.wikipedia.org/wiki/Rust_(lenguagg_de_programazzion) "Rust (lenguagg de programazzion) – Lombard"
[28]: https://hu.wikipedia.org/wiki/Rust_(programoz%C3%A1si_nyelv) "Rust (programozási nyelv) – Hungarian"
[29]: https://ml.wikipedia.org/wiki/%E0%B4%B1%E0%B4%B8%E0%B5%8D%E0%B4%B1%E0%B5%8D%E0%B4%B1%E0%B5%8D_(%E0%B4%AA%E0%B5%8D%E0%B4%B0%E0%B5%8B%E0%B4%97%E0%B5%8D%E0%B4%B0%E0%B4%BE%E0%B4%AE%E0%B4%BF%E0%B4%82%E0%B4%97%E0%B5%8D_%E0%B4%AD%E0%B4%BE%E0%B4%B7) "റസ്റ്റ് (പ്രോഗ്രാമിംഗ് ഭാഷ) – Malayalam"
[30]: https://ms.wikipedia.org/wiki/Rust_(bahasa_pengaturcaraan) "Rust (bahasa pengaturcaraan) – Malay"
[31]: https://nl.wikipedia.org/wiki/Rust_(programmeertaal) "Rust (programmeertaal) – Dutch"
[32]: https://ja.wikipedia.org/wiki/Rust_(%E3%83%97%E3%83%AD%E3%82%B0%E3%83%A9%E3%83%9F%E3%83%B3%E3%82%B0%E8%A8%80%E8%AA%9E) "Rust (プログラミング言語) – Japanese"
[33]: https://no.wikipedia.org/wiki/Rust_(programmeringsspr%C3%A5k) "Rust (programmeringsspråk) – Norwegian Bokmål"
[34]: https://pl.wikipedia.org/wiki/Rust_(j%C4%99zyk_programowania) "Rust (język programowania) – Polish"
[35]: https://pt.wikipedia.org/wiki/Rust_(linguagem_de_programa%C3%A7%C3%A3o) "Rust (linguagem de programação) – Portuguese"
[36]: https://ro.wikipedia.org/wiki/Rust_(limbaj_de_programare) "Rust (limbaj de programare) – Romanian"
[37]: https://qu.wikipedia.org/wiki/Rust "Rust – Quechua"
[38]: https://ru.wikipedia.org/wiki/Rust_(%D1%8F%D0%B7%D1%8B%D0%BA_%D0%BF%D1%80%D0%BE%D0%B3%D1%80%D0%B0%D0%BC%D0%BC%D0%B8%D1%80%D0%BE%D0%B2%D0%B0%D0%BD%D0%B8%D1%8F) "Rust (язык программирования) – Russian"
[39]: https://sq.wikipedia.org/wiki/Rust_(gjuh%C3%AB_programimi) "Rust (gjuhë programimi) – Albanian"
[40]: https://simple.wikipedia.org/wiki/Rust_(programming_language) "Rust (programming language) – Simple English"
[41]: https://sk.wikipedia.org/wiki/Rust_(programovac%C3%AD_jazyk) "Rust (programovací jazyk) – Slovak"
[42]: https://sr.wikipedia.org/wiki/Rust_(%D0%BF%D1%80%D0%BE%D0%B3%D1%80%D0%B0%D0%BC%D1%81%D0%BA%D0%B8_%D1%98%D0%B5%D0%B7%D0%B8%D0%BA) "Rust (програмски језик) – Serbian"
[43]: https://fi.wikipedia.org/wiki/Rust_(ohjelmointikieli) "Rust (ohjelmointikieli) – Finnish"
[44]: https://sv.wikipedia.org/wiki/Rust_(programspr%C3%A5k) "Rust (programspråk) – Swedish"
[45]: https://te.wikipedia.org/wiki/%E0%B0%B0%E0%B0%B8%E0%B1%8D%E0%B0%9F%E0%B1%8D_(%E0%B0%AA%E0%B1%8D%E0%B0%B0%E0%B1%8B%E0%B0%97%E0%B1%8D%E0%B0%B0%E0%B0%BE%E0%B0%AE%E0%B0%BF%E0%B0%82%E0%B0%97%E0%B1%8D_%E0%B0%AD%E0%B0%BE%E0%B0%B7) "రస్ట్ (ప్రోగ్రామింగ్ భాష) – Telugu"
[46]: https://th.wikipedia.org/wiki/%E0%B8%A0%E0%B8%B2%E0%B8%A9%E0%B8%B2%E0%B8%A3%E0%B8%B1%E0%B8%AA%E0%B8%95%E0%B9%8C "ภาษารัสต์ – Thai"
[47]: https://tr.wikipedia.org/wiki/Rust_(programlama_dili) "Rust (programlama dili) – Turkish"
[48]: https://uk.wikipedia.org/wiki/Rust_(%D0%BC%D0%BE%D0%B2%D0%B0_%D0%BF%D1%80%D0%BE%D0%B3%D1%80%D0%B0%D0%BC%D1%83%D0%B2%D0%B0%D0%BD%D0%BD%D1%8F) "Rust (мова програмування) – Ukrainian"
[49]: https://vi.wikipedia.org/wiki/Rust_(ng%C3%B4n_ng%E1%BB%AF_l%E1%BA%ADp_tr%C3%ACnh) "Rust (ngôn ngữ lập trình) – Vietnamese"
[50]: https://zh-yue.wikipedia.org/wiki/Rust "Rust – Cantonese"
[51]: https://zh.wikipedia.org/wiki/Rust_(%E7%BC%96%E7%A8%8B%E8%AF%AD%E8%A8%80) "Rust (编程语言) – Chinese"
[52]: https://dtp.wikipedia.org/wiki/Rust_(boros_tokud) "Rust (boros tokud) – Central Dusun"
[53]: https://www.wikidata.org/wiki/Special:EntityPage/Q575650#sitelinks-wikipedia "Edit interlanguage links"
[54]: //upload.wikimedia.org/wikipedia/en/thumb/9/94/Symbol_support_vote.svg/20px-Symbol_support_vote.svg.png
[55]: /wiki/Wikipedia:Good_articles* "This is a good article. Click here for more information."
[56]: //upload.wikimedia.org/wikipedia/commons/thumb/d/d5/Rust_programming_language_black_logo.svg/150px-Rust_programming_language_black_logo.svg.png
[57]: /wiki/File:Rust_programming_language_black_logo.svg
[58]: /wiki/Programming_paradigm "Programming paradigm"
[59]: /wiki/Concurrent_computing "Concurrent computing"
[60]: /wiki/Functional_programming "Functional programming"
[61]: /wiki/Generic_programming "Generic programming"
[62]: /wiki/Imperative_programming "Imperative programming"
[63]: /wiki/Structured_programming "Structured programming"
[64]: /wiki/Software_developer "Software developer"
[65]: /wiki/Software_release_life_cycle "Software release life cycle"
[66]: #cite_note-wikidata-625dfa02256b12affe5cdb18bbe05fea7b2cb7a3-v20-1
[67]: //upload.wikimedia.org/wikipedia/en/thumb/8/8a/OOjs_UI_icon_edit-ltr-progressive.svg/20px-OOjs_UI_icon_edit-ltr-progressive.svg.png
[68]: https://www.wikidata.org/wiki/Q575650?uselang=en#P348 "Edit this on Wikidata"
[69]: /wiki/Type_system "Type system"
[70]: /wiki/Affine_type_system "Affine type system"
[71]: /wiki/Type_inference "Type inference"
[72]: /wiki/Nominal_type_system "Nominal type system"
[73]: /wiki/Static_typing "Static typing"
[74]: /wiki/Strong_and_weak_typing "Strong and weak typing"
[75]: /wiki/OCaml "OCaml"
[76]: /wiki/Computing_platform "Computing platform"
[77]: /wiki/Cross-platform_software "Cross-platform software"
[78]: #cite_note-3
[79]: /wiki/Operating_system "Operating system"
[80]: #cite_note-4
[81]: /wiki/Software_license "Software license"
[82]: /wiki/MIT_License "MIT License"
[83]: /wiki/Apache_License "Apache License"
[84]: #cite_note-7
[85]: /wiki/Filename_extension "Filename extension"
[86]: https://www.rust-lang.org/
[87]: /wiki/Alef_(programming_language) "Alef (programming language)"
[88]: /wiki/BETA_(programming_language) "BETA (programming language)"
[89]: /wiki/CLU_(programming_language) "CLU (programming language)"
[90]: /wiki/C_Sharp_(programming_language) "C Sharp (programming language)"
[91]: /wiki/C%2B%2B "C++"
[92]: /wiki/Cyclone_(programming_language) "Cyclone (programming language)"
[93]: /wiki/Elm_(programming_language) "Elm (programming language)"
[94]: /wiki/Erlang_(programming_language) "Erlang (programming language)"
[95]: /wiki/Haskell "Haskell"
[96]: /wiki/Hermes_(programming_language) "Hermes (programming language)"
[97]: /wiki/Limbo_(programming_language) "Limbo (programming language)"
[98]: /wiki/Mesa_(programming_language) "Mesa (programming language)"
[99]: /wiki/Napier88 "Napier88"
[100]: /wiki/Newsqueak "Newsqueak"
[101]: /wiki/Typestate_analysis "Typestate analysis"
[102]: #cite_note-10
[103]: /wiki/Ruby_(programming_language) "Ruby (programming language)"
[104]: /wiki/Sather "Sather"
[105]: /wiki/Scheme_(programming_language) "Scheme (programming language)"
[106]: /wiki/Standard_ML "Standard ML"
[107]: /wiki/Swift_(programming_language) "Swift (programming language)"
[108]: #cite_note-11
[109]: #cite_note-influences-12
[110]: /wiki/Idris_(programming_language) "Idris (programming language)"
[111]: #cite_note-13
[112]: /wiki/Project_Verona "Project Verona"
[113]: #cite_note-Project_Verona-14
[114]: /wiki/SPARK_(programming_language) "SPARK (programming language)"
[115]: #cite_note-Jaloyan-15
[116]: #cite_note-Lattner-16
[117]: /wiki/V_(programming_language) "V (programming language)"
[118]: #cite_note-17
[119]: /wiki/Zig_(programming_language) "Zig (programming language)"
[120]: #cite_note-18
[121]: /wiki/General-purpose_programming_language "General-purpose programming language"
[122]: /wiki/Programming_language "Programming language"
[123]: /wiki/Computer_performance "Computer performance"
[124]: /wiki/Type_safety "Type safety"
[125]: /wiki/Concurrency_(computer_science) "Concurrency (computer science)"
[126]: /wiki/Memory_safety "Memory safety"
[127]: /wiki/Immutable_object "Immutable object"
[128]: /wiki/Higher-order_function "Higher-order function"
[129]: /wiki/Algebraic_data_type "Algebraic data type"
[130]: /wiki/Pattern_matching "Pattern matching"
[131]: /wiki/Object-oriented_programming "Object-oriented programming"
[132]: /wiki/Union_type "Union type"
[133]: /wiki/Reference_(computer_science) "Reference (computer science)"
[134]: /wiki/Garbage_collection_(computer_science) "Garbage collection (computer science)"
[135]: /wiki/Data_race "Data race"
[136]: /wiki/Object_lifetime "Object lifetime"
[137]: /wiki/Compiler "Compiler"
[138]: /wiki/Mozilla "Mozilla"
[139]: #Rust_Foundation
[140]: /wiki/Web_service "Web service"
[141]: /wiki/System_software "System software"
[142]: /wiki/C_(programming_language) "C (programming language)"
[143]: /wiki/Assembly_language "Assembly language"
[144]: /wiki/Linux_kernel "Linux kernel"
[145]: /w/index.php?title=Rust_(programming_language)&action=edit&section=1 "Edit section: Etymology"
[146]: /wiki/Rust_(fungus) "Rust (fungus)"
[147]: #cite_note-MITTechReview-19
[148]: /wiki/Crankset#Chainring "Crankset"
[149]: #cite_note-20
[150]: /w/index.php?title=Rust_(programming_language)&action=edit&section=2 "Edit section: History"
[151]: /w/index.php?title=Rust_(programming_language)&action=edit&section=3 "Edit section: 2006–2009: Early years"
[152]: //upload.wikimedia.org/wikipedia/commons/thumb/d/dd/MozillaCaliforniaHeadquarters.JPG/250px-MozillaCaliforniaHeadquarters.JPG
[153]: /wiki/File:MozillaCaliforniaHeadquarters.JPG
[154]: /wiki/Mountain_View,_California "Mountain View, California"
[155]: #cite_note-Klabnik2016ACMHistory-21
[156]: #cite_note-Hoare2010-22
[157]: #cite_note-23
[158]: #cite_note-OCamlCompiler-24
[159]: /w/index.php?title=Rust_(programming_language)&action=edit&section=4 "Edit section: 2009–2012: Mozilla sponsorship"
[160]: /wiki/Brendan_Eich "Brendan Eich"
[161]: /wiki/Web_browser "Web browser"
[162]: /wiki/Browser_engine "Browser engine"
[163]: /wiki/Self-hosting_(compilers) "Self-hosting (compilers)"
[164]: /wiki/LLVM "LLVM"
[165]: #cite_note-Rust0.1-25
[166]: #cite_note-27
[167]: #cite_note-Rust0.1a-28
[168]: #cite_note-ExtremeTechRust0.1-29
[169]: /w/index.php?title=Rust_(programming_language)&action=edit&section=5 "Edit section: 2012–2015: Evolution"
[170]: /wiki/Pure_function "Pure function"
[171]: #cite_note-30
[172]: /wiki/Channel_(programming) "Channel (programming)"
[173]: /wiki/Scripting_language "Scripting language"
[174]: /wiki/Dr._Dobb%27s_Journal "Dr. Dobb's Journal"
[175]: #cite_note-31
[176]: /wiki/Backward_compatibility "Backward compatibility"
[177]: /wiki/Stable_release "Stable release"
[178]: /w/index.php?title=Rust_(programming_language)&action=edit&section=6 "Edit section: 2015–2020: Servo and early adoption"
[179]: //upload.wikimedia.org/wikipedia/commons/thumb/3/39/Home_page_servo_v0.01.png/250px-Home_page_servo_v0.01.png
[180]: /wiki/File:Home_page_servo_v0.01.png
[181]: /wiki/Servo_(software) "Servo (software)"
[182]: /wiki/Samsung "Samsung"
[183]: #cite_note-32
[184]: /wiki/Firefox "Firefox"
[185]: #cite_note-33
[186]: /wiki/Gecko_(software) "Gecko (software)"
[187]: /wiki/Gecko_(software)#Quantum "Gecko (software)"
[188]: #cite_note-34
[189]: #Rustfmt
[190]: /wiki/Integrated_development_environment "Integrated development environment"
[191]: /wiki/Code_of_conduct "Code of conduct"
[192]: /wiki/IRC "IRC"
[193]: /wiki/Facebook "Facebook"
[194]: /wiki/Meta_Platforms "Meta Platforms"
[195]: /wiki/Dropbox "Dropbox"
[196]: /wiki/Ember.js "Ember.js"
[197]: /wiki/Amazon_Web_Services "Amazon Web Services"
[198]: /wiki/Energy_efficiency_in_computing "Energy efficiency in computing"
[199]: /wiki/Java_(programming_language) "Java (programming language)"
[200]: #cite_note-2017PortugalEnergyStudy-35
[201]: #cite_note-36
[202]: /w/index.php?title=Rust_(programming_language)&action=edit&section=7 "Edit section: 2020–present: Mozilla layoffs and Rust Foundation"
[203]: /wiki/COVID-19_pandemic "COVID-19 pandemic"
[204]: #cite_note-37
[205]: #cite_note-38
[206]: #cite_note-39
[207]: /wiki/Trademark "Trademark"
[208]: /wiki/Domain_name "Domain name"
[209]: #cite_note-40
[210]: /wiki/Google "Google"
[211]: /wiki/Huawei "Huawei"
[212]: /wiki/Microsoft "Microsoft"
[213]: #cite_note-41
[214]: #cite_note-42
[215]: /wiki/Blog "Blog"
[216]: /wiki/Android_Open_Source_Project "Android Open Source Project"
[217]: #cite_note-43
[218]: #cite_note-moderation-44
[219]: #cite_note-45
[220]: #cite_note-ApologizesTrademarkPolicy-46
[221]: /wiki/White_House "White House"
[222]: /wiki/Office_of_the_National_Cyber_Director "Office of the National Cyber Director"
[223]: #cite_note-WhiteHouse1-47
[224]: #cite_note-WhiteHouse2-48
[225]: #cite_note-WhiteHouseFullReport-49
[226]: #cite_note-WhiteHouse3-50
[227]: #cite_note-WhiteHouse4-51
[228]: /w/index.php?title=Rust_(programming_language)&action=edit&section=8 "Edit section: Syntax and features"
[229]: /wiki/Rust_syntax "Rust syntax"
[230]: /wiki/Syntax_(programming_languages) "Syntax (programming languages)"
[231]: #cite_note-52
[232]: #cite_note-:4-53
[233]: #cite_note-FOOTNOTEKlabnikNichols2019263-54
[234]: /wiki/Memory_layout "Memory layout"
[235]: /w/index.php?title=Rust_(programming_language)&action=edit&section=9 "Edit section: Hello World program"
[236]: /wiki/%22Hello,_World!%22_program "&quot;Hello, World!&quot; program"
[237]: /wiki/Function_(computer_programming) "Function (computer programming)"
[238]: /wiki/Macro_(computer_science) "Macro (computer science)"
[239]: #Macros
[240]: /wiki/Standard_output "Standard output"
[241]: #cite_note-FOOTNOTEKlabnikNichols20195–6-55
[242]: /wiki/Statement_(computer_science) "Statement (computer science)"
[243]: /wiki/Semicolon#Programming "Semicolon"
[244]: /w/index.php?title=Rust_(programming_language)&action=edit&section=10 "Edit section: Variables"
[245]: /wiki/Variable_(computer_science) "Variable (computer science)"
[246]: #cite_note-FOOTNOTEKlabnikNichols202332-56
[247]: #cite_note-FOOTNOTEKlabnikNichols202332–33-57
[248]: /wiki/Comment_(computer_programming) "Comment (computer programming)"
[249]: #cite_note-FOOTNOTEKlabnikNichols202349–50-58
[250]: /wiki/Variable_shadowing "Variable shadowing"
[251]: #cite_note-FOOTNOTEKlabnikNichols202334–36-59
[252]: /w/index.php?title=Rust_(programming_language)&action=edit&section=11 "Edit section: Block expressions and control flow"
[253]: /wiki/Bracket#Curly_brackets "Bracket"
[254]: #cite_note-FOOTNOTEKlabnikNichols20236,_47,_53-60
[255]: #cite_note-FOOTNOTEKlabnikNichols202347–48-61
[256]: /w/index.php?title=Rust_(programming_language)&action=edit&section=12 "Edit section: if expressions"
[257]: /wiki/Conditional_expression "Conditional expression"
[258]: #cite_note-FOOTNOTEKlabnikNichols202350–53-62
[259]: /w/index.php?title=Rust_(programming_language)&action=edit&section=13 "Edit section: while loops"
[260]: /wiki/While_loop "While loop"
[261]: #cite_note-FOOTNOTEKlabnikNichols202356-63
[262]: /w/index.php?title=Rust_(programming_language)&action=edit&section=14 "Edit section: for loops and iterators"
[263]: /wiki/For_loop "For loop"
[264]: #cite_note-FOOTNOTEKlabnikNichols202357–58-64
[265]: /wiki/Iterator "Iterator"
[266]: /w/index.php?title=Rust_(programming_language)&action=edit&section=15 "Edit section: loop and break statements"
[267]: #cite_note-FOOTNOTEKlabnikNichols202354–56-65
[268]: /w/index.php?title=Rust_(programming_language)&action=edit&section=16 "Edit section: Pattern matching"
[269]: #cite_note-FOOTNOTEKlabnikNichols2019104–109-66
[270]: /w/index.php?title=Rust_(programming_language)&action=edit&section=17 "Edit section: Types"
[271]: /wiki/Strongly_typed "Strongly typed"
[272]: /wiki/Statically_typed "Statically typed"
[273]: /wiki/Compilation_error "Compilation error"
[274]: #cite_note-FOOTNOTEKlabnikNichols201924-67
[275]: /wiki/Floating_point "Floating point"
[276]: /wiki/Literal_(computer_programming) "Literal (computer programming)"
[277]: #cite_note-FOOTNOTEKlabnikNichols201936–38-68
[278]: /w/index.php?title=Rust_(programming_language)&action=edit&section=18 "Edit section: Primitive types"
[279]: /wiki/Integer_type "Integer type"
[280]: /wiki/Signedness "Signedness"
[281]: /wiki/Bus_(computing)#Address_bus "Bus (computing)"
[282]: /wiki/32-bit_computing "32-bit computing"
[283]: #cite_note-69
[284]: #cite_note-70
[285]: /wiki/Radix "Radix"
[286]: /wiki/Binary_number "Binary number"
[287]: /wiki/Octal "Octal"
[288]: /wiki/Hexadecimal "Hexadecimal"
[289]: #cite_note-FOOTNOTEKlabnikNichols202336–38-71
[290]: /wiki/ASCII "ASCII"
[291]: #cite_note-FOOTNOTEKlabnikNichols2023502-72
[292]: /wiki/Boolean_type "Boolean type"
[293]: #cite_note-73
[294]: /wiki/Unicode_codepoint "Unicode codepoint"
[295]: /wiki/Universal_Character_Set_characters#Surrogates "Universal Character Set characters"
[296]: #cite_note-74
[297]: /wiki/IEEE_754 "IEEE 754"
[298]: /wiki/Single_precision_float "Single precision float"
[299]: /wiki/Double_precision_float "Double precision float"
[300]: #cite_note-FOOTNOTEKlabnikNichols201938–40-75
[301]: /w/index.php?title=Rust_(programming_language)&action=edit&section=19 "Edit section: Compound types"
[302]: #cite_note-FOOTNOTEKlabnikNichols202340–42-76
[303]: #cite_note-FOOTNOTEKlabnikNichols202342-77
[304]: /w/index.php?title=Rust_(programming_language)&action=edit&section=20 "Edit section: Ownership and references"
[305]: #cite_note-FOOTNOTEKlabnikNichols201959–61-78
[306]: #cite_note-FOOTNOTEKlabnikNichols201963–68-79
[307]: /wiki/Dangling_pointers "Dangling pointers"
[308]: #cite_note-FOOTNOTEKlabnikNichols201974–75-80
[309]: #cite_note-FOOTNOTEKlabnikNichols202371–72-81
[310]: /wiki/Linear_types "Linear types"
[311]: /wiki/Software_fault_isolation "Software fault isolation"
[312]: #cite_note-BeyondSafety-82
[313]: /wiki/Destructor_(computer_programming) "Destructor (computer programming)"
[314]: #Traits
[315]: /wiki/Lock_(computer_science) "Lock (computer science)"
[316]: #cite_note-FOOTNOTEKlabnikNichols2023327–30-83
[317]: /w/index.php?title=Rust_(programming_language)&action=edit&section=21 "Edit section: Lifetimes"
[318]: #cite_note-84
[319]: #cite_note-85
[320]: /wiki/Scope_(computer_science) "Scope (computer science)"
[321]: #cite_note-FOOTNOTEKlabnikNichols2019194-86
[322]: #cite_note-FOOTNOTEKlabnikNichols201975,_134-87
[323]: #cite_note-88
[324]: #cite_note-FOOTNOTEKlabnikNichols2019194–195-89
[325]: #cite_note-FOOTNOTEKlabnikNichols2023208–12-90
[326]: #cite_note-FOOTNOTEKlabnikNichols2023[httpsdocrust-langorgbookch04-02-references-and-borrowinghtml_4.2._References_and_Borrowing]-91
[327]: #cite_note-Pearce-92
[328]: /w/index.php?title=Rust_(programming_language)&action=edit&section=22 "Edit section: User-defined types"
[329]: /wiki/Record_(computer_science) "Record (computer science)"
[330]: #cite_note-FOOTNOTEKlabnikNichols201983-93
[331]: /wiki/Algebraic_data_types "Algebraic data types"
[332]: #cite_note-FOOTNOTEKlabnikNichols201997-94
[333]: /wiki/Field_(computer_science) "Field (computer science)"
[334]: #cite_note-FOOTNOTEKlabnikNichols201998–101-95
[335]: #cite_note-FOOTNOTEKlabnikNichols2019438–440-96
[336]: /wiki/Class_(computer_programming) "Class (computer programming)"
[337]: #cite_note-FOOTNOTEKlabnikNichols201993-97
[338]: /w/index.php?title=Rust_(programming_language)&action=edit&section=23 "Edit section: Standard library"
[339]: //upload.wikimedia.org/wikipedia/commons/thumb/a/af/Rust_standard_libraries.svg/250px-Rust_standard_libraries.svg.png
[340]: /wiki/File:Rust_standard_libraries.svg
[341]: /wiki/Standard_library "Standard library"
[342]: /wiki/Smart_pointer "Smart pointer"
[343]: #cite_note-FOOTNOTEGjengset2021213–215-98
[344]: /wiki/Option_type "Option type"
[345]: #cite_note-FOOTNOTEKlabnikNichols2023108–110,_113–114,_116–117-99
[346]: /wiki/Result_type "Result type"
[347]: #cite_note-Rust_error_handling-100
[348]: /w/index.php?title=Rust_(programming_language)&action=edit&section=24 "Edit section: Pointers"
[349]: #cite_note-FOOTNOTEGjengset2021155-156-101
[350]: #cite_note-FOOTNOTEKlabnikNichols2023421–423-102
[351]: #cite_note-FOOTNOTEKlabnikNichols2019418–427-103
[352]: /w/index.php?title=Rust_(programming_language)&action=edit&section=25 "Edit section: Type conversion"
[353]: /wiki/Type_conversion#Rust "Type conversion"
[354]: https://en.wikipedia.org/w/index.php?title=Type_conversion&action=edit
[355]: #cite_note-104
[356]: //upload.wikimedia.org/wikipedia/commons/transcoded/5/5c/Rust_101.webm/Rust_101.webm.480p.vp9.webm
[357]: /wiki/Linux.conf.au "Linux.conf.au"
[358]: /w/index.php?title=Rust_(programming_language)&action=edit&section=26 "Edit section: Polymorphism"
[359]: /wiki/Bounded_parametric_polymorphism "Bounded parametric polymorphism"
[360]: /wiki/Trait_(computer_programming) "Trait (computer programming)"
[361]: /wiki/Generic_function "Generic function"
[362]: #cite_note-FOOTNOTEKlabnikNichols2023378-105
[363]: #cite_note-FOOTNOTEKlabnikNichols2023192–198-106
[364]: #cite_note-FOOTNOTEKlabnikNichols202398-107
[365]: #cite_note-FOOTNOTEKlabnikNichols2019171–172,_205-108
[366]: /w/index.php?title=Rust_(programming_language)&action=edit&section=27 "Edit section: Trait objects"
[367]: /wiki/Static_dispatch "Static dispatch"
[368]: /wiki/Monomorphization "Monomorphization"
[369]: #cite_note-FOOTNOTEKlabnikNichols2023191–192-109
[370]: #cite_note-FOOTNOTEGjengset202125-110
[371]: /wiki/Dynamic_dispatch "Dynamic dispatch"
[372]: /wiki/Runtime_(program_lifecycle_phase) "Runtime (program lifecycle phase)"
[373]: /wiki/Duck_typing "Duck typing"
[374]: #cite_note-FOOTNOTEKlabnikNichols2023[httpsdocrust-langorgbookch18-02-trait-objectshtml_18.2._Using_Trait_Objects_That_Allow_for_Values_of_Different_Types]-111
[375]: #cite_note-FOOTNOTEKlabnikNichols2019441–442-112
[376]: #cite_note-FOOTNOTEKlabnikNichols2019379–380-113
[377]: /w/index.php?title=Rust_(programming_language)&action=edit&section=28 "Edit section: Memory safety"
[378]: /wiki/Memory_safe "Memory safe"
[379]: /wiki/Dangling_pointer "Dangling pointer"
[380]: #cite_note-cnet-114
[381]: #cite_note-lwn-115
[382]: #cite_note-The_Rustonomicon-116
[383]: #cite_note-Sensors-117
[384]: #cite_note-lang-faq-118
[385]: /w/index.php?title=Rust_(programming_language)&action=edit&section=29 "Edit section: Memory management"
[386]: #cite_note-119
[387]: /wiki/Reference_counting "Reference counting"
[388]: /wiki/Overhead_(computing) "Overhead (computing)"
[389]: #cite_note-120
[390]: /wiki/Stack-based_memory_allocation "Stack-based memory allocation"
[391]: /wiki/Dynamic_allocation "Dynamic allocation"
[392]: #cite_note-121
[393]: /wiki/Undefined_behavior "Undefined behavior"
[394]: #cite_note-FOOTNOTEKlabnikNichols201970–75-122
[395]: #cite_note-FOOTNOTEKlabnikNichols2019323-123
[396]: /w/index.php?title=Rust_(programming_language)&action=edit&section=30 "Edit section: Unsafe"
[397]: #cite_note-FOOTNOTEKlabnikNichols2023420–429-124
[398]: /wiki/Volatile_(computer_programming) "Volatile (computer programming)"
[399]: /wiki/Type_punning "Type punning"
[400]: #cite_note-FOOTNOTEMcNamara2021139,_376–379,_395-125
[401]: #cite_note-UnsafeRustUse-126
[402]: /wiki/Doubly_linked_list "Doubly linked list"
[403]: #cite_note-127
[404]: #cite_note-128
[405]: #cite_note-129
[406]: #cite_note-130
[407]: #cite_note-IsRustSafely-131
[408]: #cite_note-132
[409]: /w/index.php?title=Rust_(programming_language)&action=edit&section=31 "Edit section: Macros"
[410]: #cite_note-FOOTNOTEKlabnikNichols2023449–455-133
[411]: #cite_note-FOOTNOTEGjengset2021101–102-134
[412]: /w/index.php?title=Rust_(programming_language)&action=edit&section=32 "Edit section: Declarative macros"
[413]: #cite_note-Rust_Ref._–_Macros_By_Example-135
[414]: #cite_note-FOOTNOTEKlabnikNichols2019446–448-136
[415]: /w/index.php?title=Rust_(programming_language)&action=edit&section=33 "Edit section: Procedural macros"
[416]: /wiki/Token_(parser) "Token (parser)"
[417]: #cite_note-rust-procedural-macros-137
[418]: #cite_note-FOOTNOTEKlabnikNichols2019449–455-138
[419]: /w/index.php?title=Rust_(programming_language)&action=edit&section=34 "Edit section: Interface with C and C++"
[420]: /wiki/Foreign_function_interface "Foreign function interface"
[421]: /wiki/Calling_convention "Calling convention"
[422]: #cite_note-140
[423]: #cite_note-FOOTNOTEGjengset2021193–209-141
[424]: #cite_note-142
[425]: /w/index.php?title=Rust_(programming_language)&action=edit&section=35 "Edit section: Ecosystem"
[426]: //upload.wikimedia.org/wikipedia/commons/transcoded/5/52/Cargo_compiling.webm/Cargo_compiling.webm.480p.vp9.webm
[427]: #Standard_library
[428]: /wiki/Toolchain "Toolchain"
[429]: #cite_note-FOOTNOTEBlandyOrendorffTindall20216–8-143
[430]: /w/index.php?title=Rust_(programming_language)&action=edit&section=36 "Edit section: Compiler"
[431]: /wiki/Executable "Executable"
[432]: /wiki/Abstract_syntax_tree "Abstract syntax tree"
[433]: /wiki/Intermediate_representation "Intermediate representation"
[434]: /wiki/Optimizing_compiler "Optimizing compiler"
[435]: /wiki/Object_code "Object code"
[436]: /wiki/Linker_(computing) "Linker (computing)"
[437]: #cite_note-144
[438]: /wiki/GNU_Compiler_Collection "GNU Compiler Collection"
[439]: /wiki/Cranelift "Cranelift"
[440]: #cite_note-145
[441]: #cite_note-146
[442]: #cite_note-147
[443]: /w/index.php?title=Rust_(programming_language)&action=edit&section=37 "Edit section: Cargo"
[444]: //upload.wikimedia.org/wikipedia/commons/thumb/8/88/Crates.io_website.png/250px-Crates.io_website.png
[445]: /wiki/File:Crates.io_website.png
[446]: /wiki/Build_system_(software_development) "Build system (software development)"
[447]: /wiki/Package_manager "Package manager"
[448]: #cite_note-Nature-148
[449]: /wiki/Git "Git"
[450]: #cite_note-149
[451]: /w/index.php?title=Rust_(programming_language)&action=edit&section=38 "Edit section: Rustfmt"
[452]: /wiki/Code_formatter "Code formatter"
[453]: /wiki/Indentation_style "Indentation style"
[454]: /wiki/Programming_style "Programming style"
[455]: #cite_note-FOOTNOTEKlabnikNichols2019511–512-150
[456]: /w/index.php?title=Rust_(programming_language)&action=edit&section=39 "Edit section: Clippy"
[457]: //upload.wikimedia.org/wikipedia/commons/thumb/6/6c/Cargo_clippy_hello_world_example.png/250px-Cargo_clippy_hello_world_example.png
[458]: /wiki/File:Cargo_clippy_hello_world_example.png
[459]: /wiki/Linting "Linting"
[460]: https://en.wikipedia.org/w/index.php?title=Rust_(programming_language)&action=edit
[461]: #cite_note-151
[462]: #cite_note-152
[463]: /w/index.php?title=Rust_(programming_language)&action=edit&section=40 "Edit section: Versioning system"
[464]: #cite_note-Rust_Book_G-153
[465]: /wiki/Breaking_changes "Breaking changes"
[466]: /wiki/Async/await "Async/await"
[467]: #cite_note-FOOTNOTEBlandyOrendorffTindall2021176–177-154
[468]: /w/index.php?title=Rust_(programming_language)&action=edit&section=41 "Edit section: IDE support"
[469]: /wiki/Utility_software "Utility software"
[470]: /wiki/Text_editor "Text editor"
[471]: /wiki/Language_Server_Protocol "Language Server Protocol"
[472]: /wiki/Autocomplete "Autocomplete"
[473]: #cite_note-FOOTNOTEKlabnikNichols2023623-155
[474]: /w/index.php?title=Rust_(programming_language)&action=edit&section=42 "Edit section: Performance"
[475]: #cite_note-156
[476]: #cite_note-157
[477]: #cite_note-FOOTNOTEMcNamara202111-158
[478]: /wiki/Array_(data_structure) "Array (data structure)"
[479]: #cite_note-SaferAtAnySpeed-159
[480]: #cite_note-FOOTNOTEMcNamara202119,_27-160
[481]: /wiki/Zero-copy "Zero-copy"
[482]: /wiki/Parsing "Parsing"
[483]: #cite_note-161
[484]: /wiki/Method_call "Method call"
[485]: #cite_note-FOOTNOTEMcNamara202120-162
[486]: /wiki/Inline_expansion "Inline expansion"
[487]: /wiki/Function_call "Function call"
[488]: #cite_note-163
[489]: #cite_note-how-fast-is-rust-164
[490]: #cite_note-165
[491]: #cite_note-FOOTNOTEGjengset202122-166
[492]: /w/index.php?title=Rust_(programming_language)&action=edit&section=43 "Edit section: Adoption"
[493]: /wiki/Category:Rust_(programming_language)_software "Category:Rust (programming language) software"
[494]: //upload.wikimedia.org/wikipedia/commons/thumb/a/a0/Firefox_logo%2C_2019.svg/250px-Firefox_logo%2C_2019.svg.png
[495]: /wiki/File:Firefox_logo,_2019.svg
[496]: #cite_note-167
[497]: /wiki/Alphabet_Inc. "Alphabet Inc."
[498]: /wiki/Chromium_(web_browser) "Chromium (web browser)"
[499]: #cite_note-168
[500]: #cite_note-169
[501]: /wiki/Frontend_and_backend "Frontend and backend"
[502]: /wiki/OpenDNS "OpenDNS"
[503]: /wiki/Domain_Name_System "Domain Name System"
[504]: /wiki/Cisco "Cisco"
[505]: #cite_note-170
[506]: #cite_note-171
[507]: /wiki/Open_sourced "Open sourced"
[508]: /wiki/Firecracker_(software) "Firecracker (software)"
[509]: #cite_note-172
[510]: /wiki/Microsoft_Azure "Microsoft Azure"
[511]: /wiki/Internet_of_things "Internet of things"
[512]: #cite_note-173
[513]: /wiki/WebAssembly "WebAssembly"
[514]: /wiki/Kubernetes "Kubernetes"
[515]: #cite_note-174
[516]: /wiki/Cloudflare "Cloudflare"
[517]: /wiki/Content_delivery_network "Content delivery network"
[518]: /wiki/Web_proxy "Web proxy"
[519]: #cite_note-175
[520]: /wiki/Npm "Npm"
[521]: #cite_note-176
[522]: #cite_note-177
[523]: #cite_note-178
[524]: //upload.wikimedia.org/wikipedia/commons/thumb/d/d1/Rust_for_Linux_logo.svg/150px-Rust_for_Linux_logo.svg.png
[525]: /wiki/File:Rust_for_Linux_logo.svg
[526]: /wiki/Rust_for_Linux "Rust for Linux"
[527]: #cite_note-UsenixRustForLinux-179
[528]: #cite_note-180
[529]: #cite_note-181
[530]: #cite_note-182
[531]: /wiki/Minimum_viable_product "Minimum viable product"
[532]: #cite_note-183
[533]: /wiki/Android_(operating_system) "Android (operating system)"
[534]: #cite_note-184
[535]: #cite_note-185
[536]: /wiki/Windows "Windows"
[537]: #cite_note-186
[538]: /wiki/Plan_9_from_Bell_Labs "Plan 9 from Bell Labs"
[539]: #cite_note-187
[540]: /wiki/Redox_(operating_system) "Redox (operating system)"
[541]: /wiki/Microkernel "Microkernel"
[542]: #cite_note-188
[543]: #cite_note-189
[544]: #cite_note-190
[545]: /wiki/Fuchsia_(operating_system) "Fuchsia (operating system)"
[546]: #cite_note-rustmag-1-191
[547]: /wiki/Stratis_(configuration_daemon) "Stratis (configuration daemon)"
[548]: /wiki/File_system "File system"
[549]: #cite_note-192
[550]: #cite_note-193
[551]: /wiki/Desktop_environment "Desktop environment"
[552]: /wiki/System76 "System76"
[553]: #cite_note-194
[554]: /wiki/Deno_(software) "Deno (software)"
[555]: /wiki/JavaScript "JavaScript"
[556]: /wiki/TypeScript "TypeScript"
[557]: /wiki/V8_(JavaScript_engine) "V8 (JavaScript engine)"
[558]: #cite_note-195
[559]: /wiki/Ruffle_(software) "Ruffle (software)"
[560]: /wiki/SWF "SWF"
[561]: #cite_note-196
[562]: /wiki/Polkadot_(cryptocurrency) "Polkadot (cryptocurrency)"
[563]: /wiki/Blockchain "Blockchain"
[564]: /wiki/Cryptocurrency "Cryptocurrency"
[565]: #cite_note-197
[566]: /wiki/Discord "Discord"
[567]: /wiki/Instant_messaging "Instant messaging"
[568]: /wiki/File_synchronization "File synchronization"
[569]: /wiki/Stack_Overflow "Stack Overflow"
[570]: #cite_note-SO-2025-survey-198
[571]: #cite_note-199
[572]: #cite_note-200
[573]: /wiki/DARPA "DARPA"
[574]: #cite_note-201
[575]: /w/index.php?title=Rust_(programming_language)&action=edit&section=44 "Edit section: In academic research"
[576]: /wiki/Programming_language_theory "Programming language theory"
[577]: #cite_note-202
[578]: #cite_note-203
[579]: /wiki/Proceedings_of_the_International_Astronomical_Union "Proceedings of the International Astronomical Union"
[580]: #cite_note-ResearchSoftware1-204
[581]: /wiki/Nature_(journal) "Nature (journal)"
[582]: /wiki/Learning_curve "Learning curve"
[583]: /w/index.php?title=Rust_(programming_language)&action=edit&section=45 "Edit section: Community"
[584]: //upload.wikimedia.org/wikipedia/commons/thumb/2/20/Rustacean-orig-noshadow.svg/250px-Rustacean-orig-noshadow.svg.png
[585]: /wiki/File:Rustacean-orig-noshadow.svg
[586]: /wiki/Crustacean "Crustacean"
[587]: #cite_note-FOOTNOTEKlabnikNichols20194-205
[588]: #cite_note-206
[589]: /wiki/MIT_Technology_Review "MIT Technology Review"
[590]: /wiki/Queer_community "Queer community"
[591]: #cite_note-StateOfRustSurvey2024-207
[592]: /wiki/GitHub "GitHub"
[593]: #cite_note-208
[594]: #cite_note-209
[595]: /w/index.php?title=Rust_(programming_language)&action=edit&section=46 "Edit section: Rust Foundation"
[596]: //upload.wikimedia.org/wikipedia/commons/thumb/3/3d/Rust_Foundation_logo.png/250px-Rust_Foundation_logo.png
[597]: /wiki/File:Rust_Foundation_logo.png
[598]: /wiki/Mozilla_Foundation "Mozilla Foundation"
[599]: /wiki/Nonprofit_organization "Nonprofit organization"
[600]: /wiki/United_States "United States"
[601]: /wiki/Chairperson "Chairperson"
[602]: /wiki/Executive_Director "Executive Director"
[603]: http://foundation.rust-lang.org
[604]: /wiki/Membership_organization "Membership organization"
[605]: /wiki/Legal_entity "Legal entity"
[606]: #cite_note-210
[607]: #cite_note-211
[608]: #cite_note-212
[609]: #cite_note-213
[610]: /w/index.php?title=Rust_(programming_language)&action=edit&section=47 "Edit section: Governance teams"
[611]: #cite_note-214
[612]: #cite_note-215
[613]: /w/index.php?title=Rust_(programming_language)&action=edit&section=48 "Edit section: See also"
[614]: //upload.wikimedia.org/wikipedia/commons/thumb/d/df/Wikibooks-logo-en-noslogan.svg/40px-Wikibooks-logo-en-noslogan.svg.png
[615]: /wiki/File:Wikibooks-logo-en-noslogan.svg
[616]: https://en.wikibooks.org/wiki/Rust_for_the_Novice_Programmer "wikibooks:Rust for the Novice Programmer"
[617]: /wiki/Comparison_of_programming_languages "Comparison of programming languages"
[618]: /wiki/History_of_programming_languages "History of programming languages"
[619]: /wiki/List_of_programming_languages "List of programming languages"
[620]: /wiki/List_of_programming_languages_by_type "List of programming languages by type"
[621]: /wiki/List_of_Rust_software_and_tools "List of Rust software and tools"
[622]: /w/index.php?title=Rust_(programming_language)&action=edit&section=49 "Edit section: Notes"
[623]: #cite_ref-3
[624]: /wiki/X86-64 "X86-64"
[625]: /wiki/ARM_architecture_family "ARM architecture family"
[626]: /wiki/MIPS_architecture "MIPS architecture"
[627]: /wiki/RISC-V "RISC-V"
[628]: /wiki/P6_(microarchitecture) "P6 (microarchitecture)"
[629]: /wiki/AArch64 "AArch64"
[630]: /wiki/PowerPC "PowerPC"
[631]: /wiki/Linux_on_IBM_Z "Linux on IBM Z"
[632]: #cite_note-CrossPlatform-2
[633]: #cite_ref-4
[634]: /wiki/Linux "Linux"
[635]: /wiki/MacOS "MacOS"
[636]: /wiki/FreeBSD "FreeBSD"
[637]: /wiki/NetBSD "NetBSD"
[638]: /wiki/Illumos "Illumos"
[639]: /wiki/IOS "IOS"
[640]: /wiki/Haiku_(operating_system) "Haiku (operating system)"
[641]: #cite_ref-7
[642]: /wiki/MSVC "MSVC"
[643]: #cite_note-5
[644]: #cite_note-licenses-6
[645]: #cite_ref-10
[646]: /wiki/IBM "IBM"
[647]: #cite_note-Strom1983-8
[648]: #cite_note-9
[649]: /wiki/NIL_(programming_language) "NIL (programming language)"
[650]: #cite_ref-23
[651]: #cite_ref-27
[652]: #cite_note-Nelson2022RustConf-26
[653]: #cite_ref-36
[654]: #cite_ref-140
[655]: #cite_note-139
[656]: #cite_ref-200
[657]: /w/index.php?title=Rust_(programming_language)&action=edit&section=50 "Edit section: References"
[658]: /w/index.php?title=Rust_(programming_language)&action=edit&section=51 "Edit section: Book sources"
[659]: /wiki/ISBN_(identifier) "ISBN (identifier)"
[660]: /wiki/Special:BookSources/9781718501850 "Special:BookSources/9781718501850"
[661]: /wiki/OCLC_(identifier) "OCLC (identifier)"
[662]: https://search.worldcat.org/oclc/1277511986
[663]: https://books.google.com/books?id=0Vv6DwAAQBAJ
[664]: /wiki/Special:BookSources/978-1-7185-0044-0 "Special:BookSources/978-1-7185-0044-0"
[665]: /wiki/Special:BookSources/978-1-4920-5254-8 "Special:BookSources/978-1-4920-5254-8"
[666]: https://search.worldcat.org/oclc/1289839504
[667]: /wiki/Special:BookSources/978-1-6172-9455-6 "Special:BookSources/978-1-6172-9455-6"
[668]: https://search.worldcat.org/oclc/1153044639
[669]: /wiki/Special:BookSources/978-1-7185-0310-6 "Special:BookSources/978-1-7185-0310-6"
[670]: https://search.worldcat.org/oclc/1363816350
[671]: /w/index.php?title=Rust_(programming_language)&action=edit&section=52 "Edit section: Others"
[672]: #cite_ref-wikidata-625dfa02256b12affe5cdb18bbe05fea7b2cb7a3-v20_1-0
[673]: https://blog.rust-lang.org/2025/09/18/Rust-1.90.0/
[674]: #cite_ref-CrossPlatform_2-0
[675]: #cite_ref-CrossPlatform_2-1
[676]: https://doc.rust-lang.org/rustc/platform-support.html
[677]: https://web.archive.org/web/20220630164523/https://doc.rust-lang.org/rustc/platform-support.html
[678]: #cite_ref-5
[679]: https://github.com/rust-lang/rust/blob/master/COPYRIGHT
[680]: https://web.archive.org/web/20230722190056/http://github.com/rust-lang/rust/blob/master/COPYRIGHT
[681]: #cite_ref-licenses_6-0
[682]: https://www.rust-lang.org/policies/licenses
[683]: https://web.archive.org/web/20250223193908/https://www.rust-lang.org/policies/licenses
[684]: #cite_ref-Strom1983_8-0
[685]: /wiki/Doi_(identifier) "Doi (identifier)"
[686]: https://doi.org/10.1145%2F567067.567093
[687]: /wiki/Special:BookSources/0897910907 "Special:BookSources/0897910907"
[688]: /wiki/S2CID_(identifier) "S2CID (identifier)"
[689]: https://api.semanticscholar.org/CorpusID:6630704
[690]: #cite_ref-9
[691]: https://www.cs.cmu.edu/~aldrich/papers/classic/tse12-typestate.pdf
[692]: https://doi.org/10.1109%2Ftse.1986.6312929
[693]: https://api.semanticscholar.org/CorpusID:15575346
[694]: #cite_ref-11
[695]: https://blog.rust-lang.org/2016/08/10/Shape-of-errors-to-come.html
[696]: https://web.archive.org/web/20160915133745/https://blog.rust-lang.org/2016/08/10/Shape-of-errors-to-come.html
[697]: #cite_ref-influences_12-0
[698]: https://doc.rust-lang.org/reference/influences.html
[699]: https://web.archive.org/web/20231126231034/https://doc.rust-lang.org/reference/influences.html
[700]: #cite_ref-13
[701]: http://docs.idris-lang.org/en/latest/reference/uniqueness-types.html
[702]: https://web.archive.org/web/20181121072557/http://docs.idris-lang.org/en/latest/reference/uniqueness-types.html
[703]: #cite_ref-Project_Verona_14-0
[704]: https://www.zdnet.com/article/microsoft-opens-up-rust-inspired-project-verona-programming-language-on-github/
[705]: /wiki/ZDNET "ZDNET"
[706]: https://web.archive.org/web/20200117143852/https://www.zdnet.com/article/microsoft-opens-up-rust-inspired-project-verona-programming-language-on-github/
[707]: #cite_ref-Jaloyan_15-0
[708]: /wiki/ArXiv_(identifier) "ArXiv (identifier)"
[709]: https://arxiv.org/abs/1710.07047
[710]: https://arxiv.org/archive/cs.PL
[711]: #cite_ref-Lattner_16-0
[712]: http://nondot.org/sabre/
[713]: https://web.archive.org/web/20181225175312/http://nondot.org/sabre/
[714]: #cite_ref-17
[715]: https://github.com/vlang/v/blob/master/doc/docs.md#introduction
[716]: #cite_ref-18
[717]: https://www.infoworld.com/article/3113083/new-challenger-joins-rust-to-upend-c-language.html
[718]: /wiki/InfoWorld "InfoWorld"
[719]: https://web.archive.org/web/20211125104022/https://www.infoworld.com/article/3113083/new-challenger-joins-rust-to-upend-c-language.html
[720]: #cite_ref-MITTechReview_19-0
[721]: #cite_ref-MITTechReview_19-1
[722]: #cite_ref-MITTechReview_19-2
[723]: #cite_ref-MITTechReview_19-3
[724]: #cite_ref-MITTechReview_19-4
[725]: #cite_ref-MITTechReview_19-5
[726]: #cite_ref-MITTechReview_19-6
[727]: #cite_ref-MITTechReview_19-7
[728]: #cite_ref-MITTechReview_19-8
[729]: #cite_ref-MITTechReview_19-9
[730]: #cite_ref-MITTechReview_19-10
[731]: #cite_ref-MITTechReview_19-11
[732]: #cite_ref-MITTechReview_19-12
[733]: #cite_ref-MITTechReview_19-13
[734]: #cite_ref-MITTechReview_19-14
[735]: #cite_ref-MITTechReview_19-15
[736]: #cite_ref-MITTechReview_19-16
[737]: #cite_ref-MITTechReview_19-17
[738]: #cite_ref-MITTechReview_19-18
[739]: #cite_ref-MITTechReview_19-19
[740]: https://www.technologyreview.com/2023/02/14/1067869/rust-worlds-fastest-growing-programming-language/
[741]: https://web.archive.org/web/20240919102849/https://www.technologyreview.com/2023/02/14/1067869/rust-worlds-fastest-growing-programming-language/
[742]: #cite_ref-20
[743]: https://bugzilla.mozilla.org/show_bug.cgi?id=680521
[744]: /wiki/Bugzilla "Bugzilla"
[745]: https://web.archive.org/web/20240202045212/https://bugzilla.mozilla.org/show_bug.cgi?id=680521
[746]: #cite_ref-Klabnik2016ACMHistory_21-0
[747]: #cite_ref-Klabnik2016ACMHistory_21-1
[748]: #cite_ref-Klabnik2016ACMHistory_21-2
[749]: #cite_ref-Klabnik2016ACMHistory_21-3
[750]: #cite_ref-Klabnik2016ACMHistory_21-4
[751]: #cite_ref-Klabnik2016ACMHistory_21-5
[752]: #cite_ref-Klabnik2016ACMHistory_21-6
[753]: #cite_ref-Klabnik2016ACMHistory_21-7
[754]: #cite_ref-Klabnik2016ACMHistory_21-8
[755]: #cite_ref-Klabnik2016ACMHistory_21-9
[756]: #cite_ref-Klabnik2016ACMHistory_21-10
[757]: #cite_ref-Klabnik2016ACMHistory_21-11
[758]: #cite_ref-Klabnik2016ACMHistory_21-12
[759]: #cite_ref-Klabnik2016ACMHistory_21-13
[760]: #cite_ref-Klabnik2016ACMHistory_21-14
[761]: #cite_ref-Klabnik2016ACMHistory_21-15
[762]: #cite_ref-Klabnik2016ACMHistory_21-16
[763]: #cite_ref-Klabnik2016ACMHistory_21-17
[764]: #cite_ref-Klabnik2016ACMHistory_21-18
[765]: #cite_ref-Klabnik2016ACMHistory_21-19
[766]: #cite_ref-Klabnik2016ACMHistory_21-20
[767]: https://dl.acm.org/doi/10.1145/2959689.2960081
[768]: https://doi.org/10.1145%2F2959689.2960081
[769]: /wiki/Special:BookSources/978-1-4503-4464-7 "Special:BookSources/978-1-4503-4464-7"
[770]: #cite_ref-Hoare2010_22-0
[771]: #cite_ref-Hoare2010_22-1
[772]: #cite_ref-Hoare2010_22-2
[773]: https://archive.today/20211226213836/http://venge.net/graydon/talks/intro-talk-2.pdf
[774]: http://venge.net/graydon/talks/intro-talk-2.pdf
[775]: #cite_ref-OCamlCompiler_24-0
[776]: https://github.com/graydon/rust-prehistory/tree/master
[777]: #cite_ref-Rust0.1_25-0
[778]: https://github.com/rust-lang/rust/milestone/3?closed=1
[779]: #cite_ref-Nelson2022RustConf_26-0
[780]: https://www.youtube.com/watch?v=oUIjG-y4zaA
[781]: #cite_ref-Rust0.1a_28-0
[782]: https://web.archive.org/web/20120124160628/https://mail.mozilla.org/pipermail/rust-dev/2012-January/001256.html
[783]: https://mail.mozilla.org/pipermail/rust-dev/2012-January/001256.html
[784]: #cite_ref-ExtremeTechRust0.1_29-0
[785]: https://www.extremetech.com/internet/115207-mozilla-releases-rust-0-1-the-language-that-will-eventually-usurp-firefoxs-c
[786]: #cite_ref-30
[787]: https://github.com/rust-lang/rust/pull/5412
[788]: #cite_ref-31
[789]: https://web.archive.org/web/20160807075745/http://www.drdobbs.com/jvm/the-rise-and-fall-of-languages-in-2013/240165192
[790]: https://www.drdobbs.com/jvm/the-rise-and-fall-of-languages-in-2013/240165192
[791]: #cite_ref-32
[792]: https://techcrunch.com/2013/04/03/mozilla-and-samsung-collaborate-on-servo-mozillas-next-gen-browser-engine-for-tomorrows-multicore-processors/
[793]: /wiki/TechCrunch "TechCrunch"
[794]: https://web.archive.org/web/20160910211537/https://techcrunch.com/2013/04/03/mozilla-and-samsung-collaborate-on-servo-mozillas-next-gen-browser-engine-for-tomorrows-multicore-processors/
[795]: #cite_ref-33
[796]: https://www.mozilla.org/en-US/firefox/45.0/releasenotes/
[797]: https://web.archive.org/web/20160317215950/https://www.mozilla.org/en-US/firefox/45.0/releasenotes/
[798]: #cite_ref-34
[799]: https://techcrunch.com/2017/09/29/its-time-to-give-firefox-another-chance/
[800]: https://web.archive.org/web/20230815025149/https://techcrunch.com/2017/09/29/its-time-to-give-firefox-another-chance/
[801]: #cite_ref-2017PortugalEnergyStudy_35-0
[802]: https://dl.acm.org/doi/10.1145/3136014.3136031
[803]: http://repositorio.inesctec.pt/handle/123456789/5492
[804]: https://doi.org/10.1145%2F3136014.3136031
[805]: /wiki/Hdl_(identifier) "Hdl (identifier)"
[806]: https://hdl.handle.net/1822%2F65359
[807]: /wiki/Special:BookSources/978-1-4503-5525-4 "Special:BookSources/978-1-4503-5525-4"
[808]: #cite_ref-37
[809]: https://www.zdnet.com/article/mozilla-lays-off-250-employees-while-it-refocuses-on-commercial-products/
[810]: https://web.archive.org/web/20220318025804/https://www.zdnet.com/article/mozilla-lays-off-250-employees-while-it-refocuses-on-commercial-products/
[811]: #cite_ref-38
[812]: https://www.engadget.com/mozilla-firefox-250-employees-layoffs-151324924.html
[813]: /wiki/Engadget "Engadget"
[814]: https://web.archive.org/web/20201213020220/https://www.engadget.com/mozilla-firefox-250-employees-layoffs-151324924.html
[815]: #cite_ref-39
[816]: https://www.zdnet.com/article/programming-language-rust-mozilla-job-cuts-have-hit-us-badly-but-heres-how-well-survive/
[817]: https://web.archive.org/web/20220421083509/https://www.zdnet.com/article/programming-language-rust-mozilla-job-cuts-have-hit-us-badly-but-heres-how-well-survive/
[818]: #cite_ref-40
[819]: https://blog.rust-lang.org/2020/08/18/laying-the-foundation-for-rusts-future.html
[820]: https://web.archive.org/web/20201202022933/https://blog.rust-lang.org/2020/08/18/laying-the-foundation-for-rusts-future.html
[821]: #cite_ref-41
[822]: https://foundation.rust-lang.org/news/2021-02-08-hello-world/
[823]: https://web.archive.org/web/20220419124635/https://foundation.rust-lang.org/news/2021-02-08-hello-world/
[824]: #cite_ref-42
[825]: https://blog.mozilla.org/blog/2021/02/08/mozilla-welcomes-the-rust-foundation
[826]: https://web.archive.org/web/20210208212031/https://blog.mozilla.org/blog/2021/02/08/mozilla-welcomes-the-rust-foundation/
[827]: #cite_ref-43
[828]: https://arstechnica.com/gadgets/2021/04/google-is-now-writing-low-level-android-code-in-rust/
[829]: https://web.archive.org/web/20210408001446/https://arstechnica.com/gadgets/2021/04/google-is-now-writing-low-level-android-code-in-rust/
[830]: #cite_ref-moderation_44-0
[831]: https://www.theregister.com/2021/11/23/rust_moderation_team_quits/
[832]: /wiki/The_Register "The Register"
[833]: https://web.archive.org/web/20220714093245/https://www.theregister.com/2021/11/23/rust_moderation_team_quits/
[834]: #cite_ref-45
[835]: https://blog.rust-lang.org/inside-rust/2022/05/19/governance-update.html
[836]: https://web.archive.org/web/20221027030926/https://blog.rust-lang.org/inside-rust/2022/05/19/governance-update.html
[837]: #cite_ref-ApologizesTrademarkPolicy_46-0
[838]: #cite_ref-ApologizesTrademarkPolicy_46-1
[839]: https://www.theregister.com/2023/04/17/rust_foundation_apologizes_trademark_policy/
[840]: https://web.archive.org/web/20230507053637/https://www.theregister.com/2023/04/17/rust_foundation_apologizes_trademark_policy/
[841]: #cite_ref-WhiteHouse1_47-0
[842]: https://www.infoworld.com/article/2336216/white-house-urges-developers-to-dump-c-and-c.html
[843]: #cite_ref-WhiteHouse2_48-0
[844]: https://therecord.media/memory-related-software-bugs-white-house-code-report-oncd
[845]: #cite_ref-WhiteHouseFullReport_49-0
[846]: https://web.archive.org/web/20250118013136/https://www.whitehouse.gov/oncd/briefing-room/2024/02/26/press-release-technical-report/
[847]: https://www.whitehouse.gov/oncd/briefing-room/2024/02/26/press-release-technical-report/
[848]: #cite_ref-WhiteHouse3_50-0
[849]: https://www.makeuseof.com/memory-safe-programming-white-house-wants/
[850]: #cite_ref-WhiteHouse4_51-0
[851]: https://stackoverflow.blog/2024/12/30/in-rust-we-trust-white-house-office-urges-memory-safety/
[852]: #cite_ref-52
[853]: https://www.theregister.com/2021/11/19/rust_foundation_ceo/
[854]: https://web.archive.org/web/20220714110957/https://www.theregister.com/2021/11/19/rust_foundation_ceo/
[855]: #cite_ref-:4_53-0
[856]: #cite_ref-:4_53-1
[857]: #cite_ref-:4_53-2
[858]: https://web.archive.org/web/20230320172900/https://www.techrepublic.com/article/the-rust-programming-language-now-has-its-own-independent-foundation/
[859]: /wiki/TechRepublic "TechRepublic"
[860]: https://www.techrepublic.com/article/the-rust-programming-language-now-has-its-own-independent-foundation/
[861]: #cite_ref-FOOTNOTEKlabnikNichols2019263_54-0
[862]: #CITEREFKlabnikNichols2019
[863]: #cite_ref-FOOTNOTEKlabnikNichols20195–6_55-0
[864]: #cite_ref-FOOTNOTEKlabnikNichols202332_56-0
[865]: #CITEREFKlabnikNichols2023
[866]: #cite_ref-FOOTNOTEKlabnikNichols202332–33_57-0
[867]: #cite_ref-FOOTNOTEKlabnikNichols202349–50_58-0
[868]: #cite_ref-FOOTNOTEKlabnikNichols202334–36_59-0
[869]: #cite_ref-FOOTNOTEKlabnikNichols20236,_47,_53_60-0
[870]: #cite_ref-FOOTNOTEKlabnikNichols202347–48_61-0
[871]: #cite_ref-FOOTNOTEKlabnikNichols202350–53_62-0
[872]: #cite_ref-FOOTNOTEKlabnikNichols202350–53_62-1
[873]: #cite_ref-FOOTNOTEKlabnikNichols202356_63-0
[874]: #cite_ref-FOOTNOTEKlabnikNichols202357–58_64-0
[875]: #cite_ref-FOOTNOTEKlabnikNichols202354–56_65-0
[876]: #cite_ref-FOOTNOTEKlabnikNichols2019104–109_66-0
[877]: #cite_ref-FOOTNOTEKlabnikNichols201924_67-0
[878]: #cite_ref-FOOTNOTEKlabnikNichols201936–38_68-0
[879]: #cite_ref-69
[880]: https://doc.rust-lang.org/stable/std/primitive.isize.html
[881]: #cite_ref-70
[882]: https://doc.rust-lang.org/stable/std/primitive.usize.html
[883]: #cite_ref-FOOTNOTEKlabnikNichols202336–38_71-0
[884]: #cite_ref-FOOTNOTEKlabnikNichols2023502_72-0
[885]: #cite_ref-73
[886]: https://doc.rust-lang.org/std/primitive.char.html
[887]: #cite_ref-74
[888]: https://www.unicode.org/glossary/
[889]: /wiki/Unicode_Consortium "Unicode Consortium"
[890]: https://web.archive.org/web/20180924092749/http://www.unicode.org/glossary/
[891]: #cite_ref-FOOTNOTEKlabnikNichols201938–40_75-0
[892]: #cite_ref-FOOTNOTEKlabnikNichols202340–42_76-0
[893]: #cite_ref-FOOTNOTEKlabnikNichols202342_77-0
[894]: #cite_ref-FOOTNOTEKlabnikNichols201959–61_78-0
[895]: #cite_ref-FOOTNOTEKlabnikNichols201963–68_79-0
[896]: #cite_ref-FOOTNOTEKlabnikNichols201963–68_79-1
[897]: #cite_ref-FOOTNOTEKlabnikNichols201974–75_80-0
[898]: #cite_ref-FOOTNOTEKlabnikNichols202371–72_81-0
[899]: #cite_ref-BeyondSafety_82-0
[900]: #cite_ref-BeyondSafety_82-1
[901]: https://doi.org/10.1145/3102980.3103006
[902]: https://doi.org/10.1145%2F3102980.3103006
[903]: /wiki/Special:BookSources/978-1-4503-5068-6 "Special:BookSources/978-1-4503-5068-6"
[904]: https://api.semanticscholar.org/CorpusID:24100599
[905]: https://web.archive.org/web/20220611034046/https://dl.acm.org/doi/10.1145/3102980.3103006
[906]: #cite_ref-FOOTNOTEKlabnikNichols2023327–30_83-0
[907]: #cite_ref-84
[908]: https://doc.rust-lang.org/rust-by-example/scope/lifetime.html
[909]: https://web.archive.org/web/20241116192422/https://doc.rust-lang.org/rust-by-example/scope/lifetime.html
[910]: #cite_ref-85
[911]: https://doc.rust-lang.org/rust-by-example/scope/lifetime/explicit.html
[912]: #cite_ref-FOOTNOTEKlabnikNichols2019194_86-0
[913]: #cite_ref-FOOTNOTEKlabnikNichols2019194_86-1
[914]: #cite_ref-FOOTNOTEKlabnikNichols201975,_134_87-0
[915]: #cite_ref-88
[916]: https://www.infoq.com/presentations/rust-borrow-checker/
[917]: https://web.archive.org/web/20220625140128/https://www.infoq.com/presentations/rust-borrow-checker/
[918]: #cite_ref-FOOTNOTEKlabnikNichols2019194–195_89-0
[919]: #cite_ref-FOOTNOTEKlabnikNichols2023208–12_90-0
[920]: #cite_ref-FOOTNOTEKlabnikNichols2023[httpsdocrust-langorgbookch04-02-references-and-borrowinghtml_4.2._References_and_Borrowing]_91-0
[921]: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
[922]: #cite_ref-Pearce_92-0
[923]: https://dl.acm.org/doi/10.1145/3443420
[924]: https://doi.org/10.1145%2F3443420
[925]: https://web.archive.org/web/20240415053627/https://dl.acm.org/doi/10.1145/3443420
[926]: #cite_ref-FOOTNOTEKlabnikNichols201983_93-0
[927]: #cite_ref-FOOTNOTEKlabnikNichols201997_94-0
[928]: #cite_ref-FOOTNOTEKlabnikNichols201998–101_95-0
[929]: #cite_ref-FOOTNOTEKlabnikNichols2019438–440_96-0
[930]: #cite_ref-FOOTNOTEKlabnikNichols201993_97-0
[931]: #cite_ref-FOOTNOTEGjengset2021213–215_98-0
[932]: #CITEREFGjengset2021
[933]: #cite_ref-FOOTNOTEKlabnikNichols2023108–110,_113–114,_116–117_99-0
[934]: #cite_ref-Rust_error_handling_100-0
[935]: https://bitfieldconsulting.com/posts/rust-errors-option-result
[936]: https://web.archive.org/web/20250807061432/https://bitfieldconsulting.com/posts/rust-errors-option-result
[937]: #cite_ref-FOOTNOTEGjengset2021155-156_101-0
[938]: #cite_ref-FOOTNOTEKlabnikNichols2023421–423_102-0
[939]: #cite_ref-FOOTNOTEKlabnikNichols2019418–427_103-0
[940]: #cite_ref-104
[941]: https://doc.rust-lang.org/rust-by-example/types/cast.html
[942]: #cite_ref-FOOTNOTEKlabnikNichols2023378_105-0
[943]: #cite_ref-FOOTNOTEKlabnikNichols2023192–198_106-0
[944]: #cite_ref-FOOTNOTEKlabnikNichols2023192–198_106-1
[945]: #cite_ref-FOOTNOTEKlabnikNichols2023192–198_106-2
[946]: #cite_ref-FOOTNOTEKlabnikNichols202398_107-0
[947]: #cite_ref-FOOTNOTEKlabnikNichols2019171–172,_205_108-0
[948]: #cite_ref-FOOTNOTEKlabnikNichols2023191–192_109-0
[949]: #cite_ref-FOOTNOTEKlabnikNichols2023191–192_109-1
[950]: #cite_ref-FOOTNOTEGjengset202125_110-0
[951]: #cite_ref-FOOTNOTEKlabnikNichols2023[httpsdocrust-langorgbookch18-02-trait-objectshtml_18.2._Using_Trait_Objects_That_Allow_for_Values_of_Different_Types]_111-0
[952]: https://doc.rust-lang.org/book/ch18-02-trait-objects.html
[953]: #cite_ref-FOOTNOTEKlabnikNichols2019441–442_112-0
[954]: #cite_ref-FOOTNOTEKlabnikNichols2019379–380_113-0
[955]: #cite_ref-cnet_114-0
[956]: http://reviews.cnet.com/8301-3514_7-57577639/samsung-joins-mozillas-quest-for-rust/
[957]: /wiki/CNET "CNET"
[958]: https://web.archive.org/web/20130404142333/http://reviews.cnet.com/8301-3514_7-57577639/samsung-joins-mozillas-quest-for-rust/
[959]: #cite_ref-lwn_115-0
[960]: https://lwn.net/Articles/547145/
[961]: /wiki/LWN.net "LWN.net"
[962]: https://web.archive.org/web/20130426010754/http://lwn.net/Articles/547145/
[963]: #cite_ref-The_Rustonomicon_116-0
[964]: https://doc.rust-lang.org/nomicon/races.html
[965]: https://web.archive.org/web/20170710194643/https://doc.rust-lang.org/nomicon/races.html
[966]: #cite_ref-Sensors_117-0
[967]: https://www.ncbi.nlm.nih.gov/pmc/articles/PMC11398098
[968]: /wiki/Bibcode_(identifier) "Bibcode (identifier)"
[969]: https://ui.adsabs.harvard.edu/abs/2024Senso..24.5818V
[970]: https://doi.org/10.3390%2Fs24175818
[971]: /wiki/PMC_(identifier) "PMC (identifier)"
[972]: /wiki/PMID_(identifier) "PMID (identifier)"
[973]: https://pubmed.ncbi.nlm.nih.gov/39275729
[974]: #cite_ref-lang-faq_118-0
[975]: https://web.archive.org/web/20150420104147/http://static.rust-lang.org/doc/master/complement-lang-faq.html
[976]: http://static.rust-lang.org/doc/master/complement-lang-faq.html
[977]: #cite_ref-119
[978]: https://doc.rust-lang.org/rust-by-example/scope/raii.html
[979]: https://web.archive.org/web/20190421131142/https://doc.rust-lang.org/rust-by-example/scope/raii.html
[980]: #cite_ref-120
[981]: https://blog.rust-lang.org/2015/05/11/traits.html
[982]: https://web.archive.org/web/20210923101530/https://blog.rust-lang.org/2015/05/11/traits.html
[983]: #cite_ref-121
[984]: https://doc.rust-lang.org/stable/rust-by-example/std/box.html
[985]: https://web.archive.org/web/20220531114141/https://doc.rust-lang.org/stable/rust-by-example/std/box.html
[986]: #cite_ref-FOOTNOTEKlabnikNichols201970–75_122-0
[987]: #cite_ref-FOOTNOTEKlabnikNichols2019323_123-0
[988]: #cite_ref-FOOTNOTEKlabnikNichols2023420–429_124-0
[989]: #cite_ref-FOOTNOTEMcNamara2021139,_376–379,_395_125-0
[990]: #CITEREFMcNamara2021
[991]: #cite_ref-UnsafeRustUse_126-0
[992]: #cite_ref-UnsafeRustUse_126-1
[993]: https://dl.acm.org/doi/10.1145/3428204
[994]: https://doi.org/10.1145%2F3428204
[995]: https://hdl.handle.net/20.500.11850%2F465785
[996]: /wiki/ISSN_(identifier) "ISSN (identifier)"
[997]: https://search.worldcat.org/issn/2475-1421
[998]: #cite_ref-127
[999]: https://dl.acm.org/doi/10.1145/3586037
[1000]: https://doi.org/10.1145%2F3586037
[1001]: https://hdl.handle.net/20.500.11850%2F610518
[1002]: #cite_ref-128
[1003]: https://doi.org/10.1145%2F3519939.3523443
[1004]: /wiki/Special:BookSources/978-1-4503-9265-5 "Special:BookSources/978-1-4503-9265-5"
[1005]: #cite_ref-129
[1006]: https://rust-unofficial.github.io/too-many-lists/
[1007]: #cite_ref-130
[1008]: https://doi.org/10.1145/3611096.3611097
[1009]: https://doi.org/10.1145%2F3611096.3611097
[1010]: /wiki/Special:BookSources/979-8-4007-0784-1 "Special:BookSources/979-8-4007-0784-1"
[1011]: #cite_ref-IsRustSafely_131-0
[1012]: #cite_ref-IsRustSafely_131-1
[1013]: https://doi.org/10.1145/3377811.3380413
[1014]: https://arxiv.org/abs/2007.00752
[1015]: https://doi.org/10.1145%2F3377811.3380413
[1016]: /wiki/Special:BookSources/978-1-4503-7121-6 "Special:BookSources/978-1-4503-7121-6"
[1017]: #cite_ref-132
[1018]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
[1019]: #cite_ref-FOOTNOTEKlabnikNichols2023449–455_133-0
[1020]: #cite_ref-FOOTNOTEGjengset2021101–102_134-0
[1021]: #cite_ref-Rust_Ref._–_Macros_By_Example_135-0
[1022]: https://doc.rust-lang.org/reference/macros-by-example.html
[1023]: https://web.archive.org/web/20230421052332/https://doc.rust-lang.org/reference/macros-by-example.html
[1024]: #cite_ref-FOOTNOTEKlabnikNichols2019446–448_136-0
[1025]: #cite_ref-rust-procedural-macros_137-0
[1026]: https://doc.rust-lang.org/reference/procedural-macros.html
[1027]: https://web.archive.org/web/20201107233444/https://doc.rust-lang.org/reference/procedural-macros.html
[1028]: #cite_ref-FOOTNOTEKlabnikNichols2019449–455_138-0
[1029]: #cite_ref-139
[1030]: https://www.heise.de/en/background/Programming-language-Rust-2024-is-the-most-comprehensive-edition-to-date-10393917.html
[1031]: /wiki/Heise_online "Heise online"
[1032]: #cite_ref-FOOTNOTEGjengset2021193–209_141-0
[1033]: #cite_ref-FOOTNOTEGjengset2021193–209_141-1
[1034]: #cite_ref-FOOTNOTEGjengset2021193–209_141-2
[1035]: #cite_ref-142
[1036]: https://www.infoq.com/news/2020/12/cpp-rust-interop-cxx/
[1037]: https://web.archive.org/web/20210122142035/https://www.infoq.com/news/2020/12/cpp-rust-interop-cxx/
[1038]: #cite_ref-FOOTNOTEBlandyOrendorffTindall20216–8_143-0
[1039]: #CITEREFBlandyOrendorffTindall2021
[1040]: #cite_ref-144
[1041]: https://rustc-dev-guide.rust-lang.org/overview.html
[1042]: https://web.archive.org/web/20230531035222/https://rustc-dev-guide.rust-lang.org/overview.html
[1043]: #cite_ref-145
[1044]: https://rustc-dev-guide.rust-lang.org/backend/codegen.html
[1045]: #cite_ref-146
[1046]: https://github.com/rust-lang/rustc_codegen_gcc#Motivation
[1047]: #cite_ref-147
[1048]: https://github.com/rust-lang/rustc_codegen_cranelift
[1049]: #cite_ref-Nature_148-0
[1050]: #cite_ref-Nature_148-1
[1051]: #cite_ref-Nature_148-2
[1052]: https://www.nature.com/articles/d41586-020-03382-2
[1053]: https://ui.adsabs.harvard.edu/abs/2020Natur.588..185P
[1054]: https://doi.org/10.1038%2Fd41586-020-03382-2
[1055]: https://pubmed.ncbi.nlm.nih.gov/33262490
[1056]: https://api.semanticscholar.org/CorpusID:227251258
[1057]: https://web.archive.org/web/20220506040523/https://www.nature.com/articles/d41586-020-03382-2
[1058]: #cite_ref-149
[1059]: https://www.infoq.com/news/2019/04/rust-1.34-additional-registries
[1060]: https://web.archive.org/web/20220714164454/https://www.infoq.com/news/2019/04/rust-1.34-additional-registries
[1061]: #cite_ref-FOOTNOTEKlabnikNichols2019511–512_150-0
[1062]: #cite_ref-151
[1063]: https://github.com/rust-lang/rust-clippy
[1064]: https://web.archive.org/web/20210523042004/https://github.com/rust-lang/rust-clippy
[1065]: #cite_ref-152
[1066]: https://rust-lang.github.io/rust-clippy/master/index.html
[1067]: #cite_ref-Rust_Book_G_153-0
[1068]: #cite_ref-FOOTNOTEBlandyOrendorffTindall2021176–177_154-0
[1069]: #cite_ref-FOOTNOTEKlabnikNichols2023623_155-0
[1070]: #cite_ref-156
[1071]: https://www.theregister.com/2021/11/30/aws_reinvent_rust/
[1072]: https://web.archive.org/web/20220711001629/https://www.theregister.com/2021/11/30/aws_reinvent_rust/
[1073]: #cite_ref-157
[1074]: https://www.infoworld.com/article/3218074/what-is-rust-safe-fast-and-easy-software-development.html
[1075]: https://web.archive.org/web/20220624101013/https://www.infoworld.com/article/3218074/what-is-rust-safe-fast-and-easy-software-development.html
[1076]: #cite_ref-FOOTNOTEMcNamara202111_158-0
[1077]: #cite_ref-SaferAtAnySpeed_159-0
[1078]: #cite_ref-SaferAtAnySpeed_159-1
[1079]: https://doi.org/10.1145%2F3485480
[1080]: https://api.semanticscholar.org/CorpusID:238212612
[1081]: #cite_ref-FOOTNOTEMcNamara202119,_27_160-0
[1082]: #cite_ref-161
[1083]: https://doi.org/10.1109%2FSPW.2015.31
[1084]: /wiki/Special:BookSources/978-1-4799-9933-0 "Special:BookSources/978-1-4799-9933-0"
[1085]: https://api.semanticscholar.org/CorpusID:16608844
[1086]: #cite_ref-FOOTNOTEMcNamara202120_162-0
[1087]: #cite_ref-163
[1088]: https://doc.rust-lang.org/reference/attributes/codegen.html
[1089]: https://web.archive.org/web/20221009202615/https://doc.rust-lang.org/reference/attributes/codegen.html
[1090]: #cite_ref-how-fast-is-rust_164-0
[1091]: https://doc.rust-lang.org/1.0.0/complement-lang-faq.html#how-fast-is-rust?
[1092]: https://web.archive.org/web/20201028102013/https://doc.rust-lang.org/1.0.0/complement-lang-faq.html#how-fast-is-rust?
[1093]: #cite_ref-165
[1094]: https://dlnext.acm.org/doi/abs/10.1145/3445814.3446724
[1095]: https://doi.org/10.1145%2F3445814.3446724
[1096]: /wiki/Special:BookSources/9781450383172 "Special:BookSources/9781450383172"
[1097]: https://api.semanticscholar.org/CorpusID:231949599
[1098]: https://web.archive.org/web/20220712060927/https://dlnext.acm.org/doi/abs/10.1145/3445814.3446724
[1099]: #cite_ref-FOOTNOTEGjengset202122_166-0
[1100]: #cite_ref-167
[1101]: https://www.computerworld.com/article/3137050/mozilla-plans-to-rejuvenate-firefox-in-2017.html
[1102]: /wiki/Computerworld "Computerworld"
[1103]: https://web.archive.org/web/20230513020437/https://www.computerworld.com/article/3137050/mozilla-plans-to-rejuvenate-firefox-in-2017.html
[1104]: #cite_ref-168
[1105]: https://www.theregister.com/2023/01/12/google_chromium_rust/
[1106]: #cite_ref-169
[1107]: https://security.googleblog.com/2023/01/supporting-use-of-rust-in-chromium.html
[1108]: https://web.archive.org/web/20230113004438/https://security.googleblog.com/2023/01/supporting-use-of-rust-in-chromium.html
[1109]: #cite_ref-170
[1110]: https://www.cnet.com/tech/services-and-software/firefox-mozilla-gets-overhaul-in-a-bid-to-get-you-interested-again/
[1111]: https://web.archive.org/web/20220714172029/https://www.cnet.com/tech/services-and-software/firefox-mozilla-gets-overhaul-in-a-bid-to-get-you-interested-again/
[1112]: #cite_ref-171
[1113]: https://web.archive.org/web/20230513161542/https://umbrella.cisco.com/blog/zeromq-helping-us-block-malicious-domains
[1114]: https://umbrella.cisco.com/blog/zeromq-helping-us-block-malicious-domains
[1115]: #cite_ref-172
[1116]: https://www.zdnet.com/article/aws-to-sponsor-rust-project/
[1117]: #cite_ref-173
[1118]: https://www.theregister.co.uk/2018/06/27/microsofts_next_cloud_trick_kicking_things_out_of_the_cloud_to_azure_iot_edge/
[1119]: https://web.archive.org/web/20190927092433/https://www.theregister.co.uk/2018/06/27/microsofts_next_cloud_trick_kicking_things_out_of_the_cloud_to_azure_iot_edge/
[1120]: #cite_ref-174
[1121]: https://www.zdnet.com/article/microsoft-why-we-used-programming-language-rust-over-go-for-webassembly-on-kubernetes-app/
[1122]: https://web.archive.org/web/20220421043549/https://www.zdnet.com/article/microsoft-why-we-used-programming-language-rust-over-go-for-webassembly-on-kubernetes-app/
[1123]: #cite_ref-175
[1124]: https://www.theregister.com/2022/09/20/rust_microsoft_c/
[1125]: #cite_ref-176
[1126]: https://www.infoq.com/news/2019/03/rust-npm-performance/
[1127]: https://web.archive.org/web/20231119135434/https://www.infoq.com/news/2019/03/rust-npm-performance/
[1128]: #cite_ref-177
[1129]: https://doi.org/10.1007/978-1-4842-5599-5_1
[1130]: https://doi.org/10.1007%2F978-1-4842-5599-5_1
[1131]: /wiki/Special:BookSources/978-1-4842-5599-5 "Special:BookSources/978-1-4842-5599-5"
[1132]: #cite_ref-178
[1133]: https://doi.org/10.1007/978-1-4842-6589-5_1
[1134]: https://doi.org/10.1007%2F978-1-4842-6589-5_1
[1135]: /wiki/Special:BookSources/978-1-4842-6589-5 "Special:BookSources/978-1-4842-6589-5"
[1136]: #cite_ref-UsenixRustForLinux_179-0
[1137]: #cite_ref-UsenixRustForLinux_179-1
[1138]: #cite_ref-UsenixRustForLinux_179-2
[1139]: https://www.usenix.org/publications/loginonline/empirical-study-rust-linux-success-dissatisfaction-and-compromise
[1140]: /wiki/USENIX "USENIX"
[1141]: #cite_ref-180
[1142]: https://lwn.net/Articles/910762/
[1143]: https://web.archive.org/web/20231117141103/https://lwn.net/Articles/910762/
[1144]: #cite_ref-181
[1145]: https://www.zdnet.com/article/rust-takes-a-major-step-forward-as-linuxs-second-official-language/
[1146]: #cite_ref-182
[1147]: https://lwn.net/Articles/914458/
[1148]: #cite_ref-183
[1149]: https://lwn.net/Articles/991062/
[1150]: #cite_ref-184
[1151]: #cite_ref-185
[1152]: https://web.archive.org/web/20210825165930/https://blog.desdelinux.net/en/google-develops-a-new-bluetooth-stack-for-android-written-in-rust/
[1153]: https://blog.desdelinux.net/en/google-develops-a-new-bluetooth-stack-for-android-written-in-rust/
[1154]: #cite_ref-186
[1155]: https://www.theregister.com/2023/04/27/microsoft_windows_rust/
[1156]: https://web.archive.org/web/20230513082735/https://www.theregister.com/2023/04/27/microsoft_windows_rust/
[1157]: #cite_ref-187
[1158]: https://www.theregister.com/2023/12/01/9front_humanbiologics/
[1159]: #cite_ref-188
[1160]: https://web.archive.org/web/20160321192838/http://www.infoworld.com/article/3046100/open-source-tools/rusts-redox-os-could-show-linux-a-few-new-tricks.html
[1161]: http://www.infoworld.com/article/3046100/open-source-tools/rusts-redox-os-could-show-linux-a-few-new-tricks.html
[1162]: #cite_ref-189
[1163]: https://www.theregister.com/2021/01/14/rust_os_theseus/
[1164]: https://web.archive.org/web/20220714112619/https://www.theregister.com/2021/01/14/rust_os_theseus/
[1165]: #cite_ref-190
[1166]: https://www.usenix.org/conference/osdi20/presentation/boos
[1167]: /wiki/Special:BookSources/978-1-939133-19-9 "Special:BookSources/978-1-939133-19-9"
[1168]: https://web.archive.org/web/20230513164135/https://www.usenix.org/conference/osdi20/presentation/boos
[1169]: #cite_ref-rustmag-1_191-0
[1170]: https://rustmagazine.org/issue-1/2022-review-the-adoption-of-rust-in-business/
[1171]: #cite_ref-192
[1172]: https://www.marksei.com/fedora-29-new-features-startis/
[1173]: https://web.archive.org/web/20190413075055/https://www.marksei.com/fedora-29-new-features-startis/
[1174]: #cite_ref-193
[1175]: https://www.theregister.com/2022/07/12/oracle_linux_9/
[1176]: https://web.archive.org/web/20220714073400/https://www.theregister.com/2022/07/12/oracle_linux_9/
[1177]: #cite_ref-194
[1178]: https://www.theregister.com/2023/02/02/system76_cosmic_xfce_updates/
[1179]: https://web.archive.org/web/20240717145511/https://www.theregister.com/2023/02/02/system76_cosmic_xfce_updates/
[1180]: #cite_ref-195
[1181]: https://www.infoq.com/news/2020/06/deno-1-ready-production/
[1182]: https://web.archive.org/web/20200701105007/https://www.infoq.com/news/2020/06/deno-1-ready-production/
[1183]: #cite_ref-196
[1184]: https://www.bleepingcomputer.com/news/software/this-flash-player-emulator-lets-you-securely-play-your-old-games/
[1185]: /wiki/Bleeping_Computer "Bleeping Computer"
[1186]: https://web.archive.org/web/20211225124131/https://www.bleepingcomputer.com/news/software/this-flash-player-emulator-lets-you-securely-play-your-old-games/
[1187]: #cite_ref-197
[1188]: https://www.bloomberg.com/news/articles/2020-10-17/ethereum-blockchain-killer-goes-by-unassuming-name-of-polkadot
[1189]: /wiki/Bloomberg_News "Bloomberg News"
[1190]: /wiki/Bloomberg_L.P. "Bloomberg L.P."
[1191]: https://web.archive.org/web/20201017192915/https://www.bloomberg.com/news/articles/2020-10-17/ethereum-blockchain-killer-goes-by-unassuming-name-of-polkadot
[1192]: #cite_ref-SO-2025-survey_198-0
[1193]: #cite_ref-SO-2025-survey_198-1
[1194]: #cite_ref-SO-2025-survey_198-2
[1195]: https://survey.stackoverflow.co/2025/technology
[1196]: #cite_ref-199
[1197]: https://www.theregister.com/2022/06/23/linus_torvalds_rust_linux_kernel/
[1198]: https://web.archive.org/web/20220728221531/https://www.theregister.com/2022/06/23/linus_torvalds_rust_linux_kernel/
[1199]: #cite_ref-201
[1200]: https://www.darpa.mil/research/programs/translating-all-c-to-rust
[1201]: #cite_ref-202
[1202]: https://dl.acm.org/doi/10.1145/3158154
[1203]: https://doi.org/10.1145%2F3158154
[1204]: https://hdl.handle.net/21.11116%2F0000-0003-34C6-3
[1205]: #cite_ref-203
[1206]: #cite_ref-ResearchSoftware1_204-0
[1207]: https://www.cambridge.org/core/journals/proceedings-of-the-international-astronomical-union/article/what-can-the-programming-language-rust-do-for-astrophysics/B51B6DF72B7641F2352C05A502F3D881
[1208]: https://arxiv.org/abs/1702.02951
[1209]: https://ui.adsabs.harvard.edu/abs/2017IAUS..325..341B
[1210]: https://doi.org/10.1017%2FS1743921316013168
[1211]: https://search.worldcat.org/issn/1743-9213
[1212]: https://api.semanticscholar.org/CorpusID:7857871
[1213]: https://web.archive.org/web/20220625140128/https://www.cambridge.org/core/journals/proceedings-of-the-international-astronomical-union/article/what-can-the-programming-language-rust-do-for-astrophysics/B51B6DF72B7641F2352C05A502F3D881
[1214]: #cite_ref-FOOTNOTEKlabnikNichols20194_205-0
[1215]: #cite_ref-206
[1216]: https://www.rust-lang.org/learn/get-started#ferris
[1217]: https://web.archive.org/web/20201101145703/https://www.rust-lang.org/learn/get-started#ferris
[1218]: #cite_ref-StateOfRustSurvey2024_207-0
[1219]: https://blog.rust-lang.org/2025/02/13/2024-State-Of-Rust-Survey-results.html
[1220]: #cite_ref-208
[1221]: https://octoverse.github.com/2022/top-programming-languages
[1222]: #cite_ref-209
[1223]: https://github.blog/news-insights/octoverse/octoverse-2024/
[1224]: #cite_ref-210
[1225]: https://www.zdnet.com/article/the-rust-programming-language-just-took-a-huge-step-forwards/
[1226]: https://web.archive.org/web/20220714105527/https://www.zdnet.com/article/the-rust-programming-language-just-took-a-huge-step-forwards/
[1227]: #cite_ref-211
[1228]: https://www.infoworld.com/article/3606774/rust-language-moves-to-independent-foundation.html
[1229]: https://web.archive.org/web/20210410161528/https://www.infoworld.com/article/3606774/rust-language-moves-to-independent-foundation.html
[1230]: #cite_ref-212
[1231]: https://www.zdnet.com/article/awss-shane-miller-to-head-the-newly-created-rust-foundation/
[1232]: https://web.archive.org/web/20210410031305/https://www.zdnet.com/article/awss-shane-miller-to-head-the-newly-created-rust-foundation/
[1233]: #cite_ref-213
[1234]: https://www.zdnet.com/article/rust-foundation-appoints-rebecca-rumbul-as-executive-director/
[1235]: https://web.archive.org/web/20211118062346/https://www.zdnet.com/article/rust-foundation-appoints-rebecca-rumbul-as-executive-director/
[1236]: #cite_ref-214
[1237]: https://www.rust-lang.org/governance
[1238]: https://web.archive.org/web/20220510225505/https://www.rust-lang.org/governance
[1239]: #cite_ref-215
[1240]: https://blog.rust-lang.org/2023/06/20/introducing-leadership-council.html
[1241]: /w/index.php?title=Rust_(programming_language)&action=edit&section=53 "Edit section: External links"
[1242]: /wiki/Wikipedia:Wikimedia_sister_projects "Wikipedia:Wikimedia sister projects"
[1243]: //upload.wikimedia.org/wikipedia/en/thumb/4/4a/Commons-logo.svg/20px-Commons-logo.svg.png
[1244]: /wiki/File:Commons-logo.svg
[1245]: https://commons.wikimedia.org/wiki/Category:Rust_(programming_language) "c:Category:Rust (programming language)"
[1246]: //upload.wikimedia.org/wikipedia/commons/thumb/0/0b/Wikiversity_logo_2017.svg/40px-Wikiversity_logo_2017.svg.png
[1247]: /wiki/File:Wikiversity_logo_2017.svg
[1248]: https://en.wikiversity.org/wiki/Rust "v:Rust"
[1249]: //upload.wikimedia.org/wikipedia/commons/thumb/f/ff/Wikidata-logo.svg/40px-Wikidata-logo.svg.png
[1250]: /wiki/File:Wikidata-logo.svg
[1251]: https://www.wikidata.org/wiki/Q575650 "d:Q575650"
[1252]: https://www.wikidata.org/wiki/Q575650#P856 "Edit this at Wikidata"
[1253]: https://github.com/rust-lang/rust
[1254]: https://doc.rust-lang.org/stable/
[1255]: /wiki/Template:Programming_languages "Template:Programming languages"
[1256]: /wiki/Template_talk:Programming_languages "Template talk:Programming languages"
[1257]: /wiki/Special:EditPage/Template:Programming_languages "Special:EditPage/Template:Programming languages"
[1258]: /wiki/Timeline_of_programming_languages "Timeline of programming languages"
[1259]: /wiki/Ada_(programming_language) "Ada (programming language)"
[1260]: /wiki/ALGOL "ALGOL"
[1261]: /wiki/Simula "Simula"
[1262]: /wiki/APL_(programming_language) "APL (programming language)"
[1263]: /wiki/BASIC "BASIC"
[1264]: /wiki/Visual_Basic "Visual Basic"
[1265]: /wiki/Visual_Basic_(classic) "Visual Basic (classic)"
[1266]: /wiki/Visual_Basic_(.NET) "Visual Basic (.NET)"
[1267]: /wiki/COBOL "COBOL"
[1268]: /wiki/Elixir_(programming_language) "Elixir (programming language)"
[1269]: /wiki/Forth_(programming_language) "Forth (programming language)"
[1270]: /wiki/Fortran "Fortran"
[1271]: /wiki/Go_(programming_language) "Go (programming language)"
[1272]: /wiki/Julia_(programming_language) "Julia (programming language)"
[1273]: /wiki/Kotlin_(programming_language) "Kotlin (programming language)"
[1274]: /wiki/Lisp_(programming_language) "Lisp (programming language)"
[1275]: /wiki/Lua "Lua"
[1276]: /wiki/MATLAB "MATLAB"
[1277]: /wiki/ML_(programming_language) "ML (programming language)"
[1278]: /wiki/Caml "Caml"
[1279]: /wiki/Pascal_(programming_language) "Pascal (programming language)"
[1280]: /wiki/Object_Pascal "Object Pascal"
[1281]: /wiki/Perl "Perl"
[1282]: /wiki/Raku_(programming_language) "Raku (programming language)"
[1283]: /wiki/PHP "PHP"
[1284]: /wiki/Prolog "Prolog"
[1285]: /wiki/Python_(programming_language) "Python (programming language)"
[1286]: /wiki/R_(programming_language) "R (programming language)"
[1287]: /wiki/SAS_language "SAS language"
[1288]: /wiki/SQL "SQL"
[1289]: /wiki/Scratch_(programming_language) "Scratch (programming language)"
[1290]: /wiki/Shell_script "Shell script"
[1291]: /wiki/Smalltalk "Smalltalk"
[1292]: //upload.wikimedia.org/wikipedia/en/thumb/d/db/Symbol_list_class.svg/20px-Symbol_list_class.svg.png
[1293]: /wiki/Generational_list_of_programming_languages "Generational list of programming languages"
[1294]: /wiki/Non-English-based_programming_languages "Non-English-based programming languages"
[1295]: //upload.wikimedia.org/wikipedia/en/thumb/9/96/Symbol_category_class.svg/20px-Symbol_category_class.svg.png
[1296]: /wiki/Category:Programming_languages "Category:Programming languages"
[1297]: /wiki/Template:Mozilla "Template:Mozilla"
[1298]: /wiki/Template_talk:Mozilla "Template talk:Mozilla"
[1299]: /wiki/Special:EditPage/Template:Mozilla "Special:EditPage/Template:Mozilla"
[1300]: /wiki/ChatZilla "ChatZilla"
[1301]: /wiki/Jetpack_(Firefox_project) "Jetpack (Firefox project)"
[1302]: /wiki/Lightning_(software) "Lightning (software)"
[1303]: /wiki/Mozilla_Persona "Mozilla Persona"
[1304]: /wiki/Mozilla_Prism "Mozilla Prism"
[1305]: /wiki/Mozilla_Raindrop "Mozilla Raindrop"
[1306]: /wiki/Mozilla_Skywriter "Mozilla Skywriter"
[1307]: /wiki/Mozilla_Sunbird "Mozilla Sunbird"
[1308]: /wiki/PDF.js "PDF.js"
[1309]: /wiki/Ubiquity_(Firefox) "Ubiquity (Firefox)"
[1310]: /wiki/Alliance_for_Open_Media "Alliance for Open Media"
[1311]: /wiki/Shumway_(software) "Shumway (software)"
[1312]: /wiki/WebXR "WebXR"
[1313]: /wiki/Asm.js "Asm.js"
[1314]: /wiki/Daala "Daala"
[1315]: /wiki/Firefox_OS "Firefox OS"
[1316]: /wiki/OpenFlint "OpenFlint"
[1317]: /wiki/Mozilla_Location_Service "Mozilla Location Service"
[1318]: /wiki/SeaMonkey "SeaMonkey"
[1319]: /wiki/Mozilla_Monitor "Mozilla Monitor"
[1320]: /wiki/Mozilla_Thunderbird "Mozilla Thunderbird"
[1321]: /wiki/Mozilla_VPN "Mozilla VPN"
[1322]: /wiki/List_of_Mozilla_products "List of Mozilla products"
[1323]: /wiki/Firefox_early_version_history "Firefox early version history"
[1324]: /wiki/Firefox_2 "Firefox 2"
[1325]: /wiki/Firefox_3.0 "Firefox 3.0"
[1326]: /wiki/Firefox_3.5 "Firefox 3.5"
[1327]: /wiki/Firefox_3.6 "Firefox 3.6"
[1328]: /wiki/Firefox_4 "Firefox 4"
[1329]: /wiki/Firefox_version_history "Firefox version history"
[1330]: /wiki/Firefox_for_Android "Firefox for Android"
[1331]: /wiki/Firefox_Focus "Firefox Focus"
[1332]: /wiki/Firefox_Sync "Firefox Sync"
[1333]: /wiki/Pocket_(service) "Pocket (service)"
[1334]: /wiki/Mozilla_Application_Suite "Mozilla Application Suite"
[1335]: /wiki/Netscape_Navigator "Netscape Navigator"
[1336]: /wiki/Netscape_Communicator "Netscape Communicator"
[1337]: /wiki/Netscape "Netscape"
[1338]: /wiki/Beonex_Communicator "Beonex Communicator"
[1339]: /wiki/Add-on_(Mozilla) "Add-on (Mozilla)"
[1340]: /wiki/Mozilla_application_framework "Mozilla application framework"
[1341]: /wiki/NPAPI "NPAPI"
[1342]: /wiki/Mozilla_Composer "Mozilla Composer"
[1343]: /wiki/Netscape_Portable_Runtime "Netscape Portable Runtime"
[1344]: /wiki/Network_Security_Services "Network Security Services"
[1345]: /wiki/Rhino_(JavaScript_engine) "Rhino (JavaScript engine)"
[1346]: /wiki/SpiderMonkey "SpiderMonkey"
[1347]: /wiki/Tamarin_(software) "Tamarin (software)"
[1348]: /wiki/Fira_(typeface) "Fira (typeface)"
[1349]: /wiki/Zilla_Slab "Zilla Slab"
[1350]: /wiki/Mozilla_Calendar_Project "Mozilla Calendar Project"
[1351]: /wiki/Camino_(web_browser) "Camino (web browser)"
[1352]: /wiki/Firefox_Lockwise "Firefox Lockwise"
[1353]: /wiki/Firefox_Send "Firefox Send"
[1354]: /wiki/Minimo "Minimo"
[1355]: /wiki/XUL "XUL"
[1356]: /wiki/XBL "XBL"
[1357]: /wiki/XPCOM "XPCOM"
[1358]: /wiki/XPInstall "XPInstall"
[1359]: /wiki/XULRunner "XULRunner"
[1360]: /wiki/Basilisk_(web_browser) "Basilisk (web browser)"
[1361]: /wiki/Classilla "Classilla"
[1362]: /wiki/Flock_(web_browser) "Flock (web browser)"
[1363]: /wiki/Floorp "Floorp"
[1364]: /wiki/Goanna_(software) "Goanna (software)"
[1365]: /wiki/GNU_IceCat "GNU IceCat"
[1366]: /wiki/LibreWolf "LibreWolf"
[1367]: /wiki/Miro_(video_software) "Miro (video software)"
[1368]: /wiki/Netscape_Navigator_9 "Netscape Navigator 9"
[1369]: /wiki/Pale_Moon "Pale Moon"
[1370]: /wiki/Firefox_Portable "Firefox Portable"
[1371]: /wiki/Swiftfox "Swiftfox"
[1372]: /wiki/Swiftweasel "Swiftweasel"
[1373]: /wiki/Waterfox "Waterfox"
[1374]: /wiki/XB_Browser "XB Browser"
[1375]: /wiki/Zen_Browser "Zen Browser"
[1376]: /wiki/Mozilla_Corporation "Mozilla Corporation"
[1377]: /wiki/Mozilla_Messaging "Mozilla Messaging"
[1378]: /wiki/Mozilla_China "Mozilla China"
[1379]: /wiki/Mozilla_Europe "Mozilla Europe"
[1380]: /wiki/Mozilla_Japan "Mozilla Japan"
[1381]: /wiki/Mitchell_Baker "Mitchell Baker"
[1382]: /wiki/David_Baron_(computer_scientist) "David Baron (computer scientist)"
[1383]: /wiki/Tantek_%C3%87elik "Tantek Çelik"
[1384]: /wiki/Laura_Chambers "Laura Chambers"
[1385]: /wiki/John_Hammink "John Hammink"
[1386]: /wiki/Johnny_Stenb%C3%A4ck "Johnny Stenbäck"
[1387]: /wiki/Doug_Turner_(Mozilla) "Doug Turner (Mozilla)"
[1388]: /wiki/Mozdev.org "Mozdev.org"
[1389]: /wiki/MDN_Web_Docs "MDN Web Docs"
[1390]: /wiki/MozillaZine "MozillaZine"
[1391]: /wiki/Mozilla_Manifesto "Mozilla Manifesto"
[1392]: /wiki/The_Book_of_Mozilla "The Book of Mozilla"
[1393]: /wiki/Code_Rush "Code Rush"
[1394]: /wiki/Mozilla_Public_License "Mozilla Public License"
[1395]: /wiki/Mozilla_(mascot) "Mozilla (mascot)"
[1396]: /wiki/Debian%E2%80%93Mozilla_trademark_dispute "Debian–Mozilla trademark dispute"
[1397]: /wiki/Common_Voice "Common Voice"
[1398]: /wiki/Mozilla_Corp._v._FCC "Mozilla Corp. v. FCC"
[1399]: /wiki/Help:Authority_control "Help:Authority control"
[1400]: https://www.wikidata.org/wiki/Q575650#identifiers "Edit this at Wikidata"
[1401]: https://d-nb.info/gnd/1078438080
[1402]: https://id.worldcat.org/fast/2002371
[1403]: https://id.loc.gov/authorities/sh2018000672
[1404]: https://catalogue.bnf.fr/ark:/12148/cb17808721b
[1405]: https://data.bnf.fr/ark:/12148/cb17808721b
[1406]: https://aleph.nkp.cz/F/?func=find-c&local_base=aut&ccl_term=ica=ph1269448&CON_LNG=ENG
[1407]: https://datos.bne.es/resource/XX6456854
[1408]: https://www.nli.org.il/en/authorities/987012402011505171
[1409]: https://lux.collections.yale.edu/view/concept/9bb5bffe-7522-4b18-9057-6d989ea6c162
[1410]: /wiki/Wikipedia:Contents/Portals "Wikipedia:Contents/Portals"
[1411]: //upload.wikimedia.org/wikipedia/commons/thumb/6/6f/Octicons-terminal.svg/20px-Octicons-terminal.svg.png
[1412]: /wiki/File:Octicons-terminal.svg
[1413]: /wiki/Portal:Computer_programming "Portal:Computer programming"
[1414]: https://en.wikipedia.org/wiki/Special:CentralAutoLogin/start?type=1x1&amp;usesul3=1
[1415]: /wiki/Help:Category "Help:Category"
[1416]: /wiki/Category:Rust_(programming_language) "Category:Rust (programming language)"
[1417]: /wiki/Category:Compiled_programming_languages "Category:Compiled programming languages"
[1418]: /wiki/Category:Concurrent_programming_languages "Category:Concurrent programming languages"
[1419]: /wiki/Category:Free_and_open_source_compilers "Category:Free and open source compilers"
[1420]: /wiki/Category:Free_software_projects "Category:Free software projects"
[1421]: /wiki/Category:Functional_languages "Category:Functional languages"
[1422]: /wiki/Category:High-level_programming_languages "Category:High-level programming languages"
[1423]: /wiki/Category:Mozilla "Category:Mozilla"
[1424]: /wiki/Category:Multi-paradigm_programming_languages "Category:Multi-paradigm programming languages"
[1425]: /wiki/Category:Pattern_matching_programming_languages "Category:Pattern matching programming languages"
[1426]: /wiki/Category:Procedural_programming_languages "Category:Procedural programming languages"
[1427]: /wiki/Category:Programming_languages_created_in_2015 "Category:Programming languages created in 2015"
[1428]: /wiki/Category:Software_using_the_Apache_license "Category:Software using the Apache license"
[1429]: /wiki/Category:Software_using_the_MIT_license "Category:Software using the MIT license"
[1430]: /wiki/Category:Statically_typed_programming_languages "Category:Statically typed programming languages"
[1431]: /wiki/Category:Systems_programming_languages "Category:Systems programming languages"
[1432]: /wiki/Category:Articles_with_short_description "Category:Articles with short description"
[1433]: /wiki/Category:Short_description_is_different_from_Wikidata "Category:Short description is different from Wikidata"
[1434]: /wiki/Category:Good_articles "Category:Good articles"
[1435]: /wiki/Category:Use_American_English_from_July_2022 "Category:Use American English from July 2022"
[1436]: /wiki/Category:All_Wikipedia_articles_written_in_American_English "Category:All Wikipedia articles written in American English"
[1437]: /wiki/Category:Use_mdy_dates_from_July_2022 "Category:Use mdy dates from July 2022"
[1438]: /wiki/Category:Articles_with_example_C%2B%2B_code "Category:Articles with example C++ code"
[1439]: /wiki/Category:Articles_with_excerpts "Category:Articles with excerpts"
[1440]: /wiki/Category:Articles_containing_potentially_dated_statements_from_2024 "Category:Articles containing potentially dated statements from 2024"
[1441]: /wiki/Category:All_articles_containing_potentially_dated_statements "Category:All articles containing potentially dated statements"
[1442]: /wiki/Category:Articles_containing_potentially_dated_statements_from_July_2024 "Category:Articles containing potentially dated statements from July 2024"
[1443]: /wiki/Category:Articles_with_example_Rust_code "Category:Articles with example Rust code"
[1444]: /wiki/Wikipedia:Text_of_the_Creative_Commons_Attribution-ShareAlike_4.0_International_License "Wikipedia:Text of the Creative Commons Attribution-ShareAlike 4.0 International License"
[1445]: https://foundation.wikimedia.org/wiki/Special:MyLanguage/Policy:Terms_of_Use "foundation:Special:MyLanguage/Policy:Terms of Use"
[1446]: https://foundation.wikimedia.org/wiki/Special:MyLanguage/Policy:Privacy_policy "foundation:Special:MyLanguage/Policy:Privacy policy"
[1447]: https://wikimediafoundation.org/
[1448]: https://foundation.wikimedia.org/wiki/Special:MyLanguage/Policy:Privacy_policy
[1449]: /wiki/Wikipedia:About
[1450]: /wiki/Wikipedia:General_disclaimer
[1451]: //en.wikipedia.org/wiki/Wikipedia:Contact_us
[1452]: https://foundation.wikimedia.org/wiki/Special:MyLanguage/Policy:Universal_Code_of_Conduct
[1453]: https://developer.wikimedia.org
[1454]: https://stats.wikimedia.org/#/en.wikipedia.org
[1455]: https://foundation.wikimedia.org/wiki/Special:MyLanguage/Policy:Cookie_statement
[1456]: //en.m.wikipedia.org/w/index.php?title=Rust_(programming_language)&mobileaction=toggle_view_mobile
[1457]: /static/images/footer/wikimedia.svg
[1458]: https://www.wikimedia.org/
[1459]: /w/resources/assets/mediawiki_compact.svg
[1460]: https://www.mediawiki.org/
[1461]: #
