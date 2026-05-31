---
canonical: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array
meta-description: The Array object, as with arrays in other programming languages, enables storing a collection of multiple items under a single variable name, and has members for performing common array operations.
meta-og:description: The Array object, as with arrays in other programming languages, enables storing a collection of multiple items under a single variable name, and has members for performing common array operations.
meta-og:image: https://developer.mozilla.org/mdn-social-image.46ac2375.png
meta-og:image:alt: The MDN logo
meta-og:image:height: 1024
meta-og:image:type: image/png
meta-og:image:width: 1024
meta-og:locale: en_US
meta-og:site_name: MDN Web Docs
meta-og:title: Array - JavaScript | MDN
meta-og:url: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array
meta-twitter:card: summary
meta-twitter:creator: MozDevNet
meta-viewport: width=device-width, initial-scale=1.0
title: Array - JavaScript | MDN
---


- [Skip to main content][1]
- [Skip to search][2]


1. [Web][3]
2. [JavaScript][4]
3. [Reference][5]
4. [Standard built-in objects][6]
5. [Array][7]

## Array

**Baseline Widely available ***

This feature is well established and works across many devices and browser versions. It’s been available across browsers since July 2015.

- Some parts of this feature may have varying levels of support.

- [Learn more][8]
- [See full compatibility][9]
- [Report feedback][10]



The **`Array`** object, as with arrays in other programming languages, enables [storing a collection of multiple items under a single variable name][11], and has members for [performing common array operations][12].

### [Description][13]

In JavaScript, arrays aren't [primitives][14] but are instead `Array` objects with the following core characteristics:

- **JavaScript arrays are resizable** and **can contain a mix of different [data types][15]**. (When those characteristics are undesirable, use [typed arrays][16] instead.)
- **JavaScript arrays are not associative arrays** and so, array elements cannot be accessed using arbitrary strings as indexes, but must be accessed using nonnegative integers (or their respective string form) as indexes.
- **JavaScript arrays are [zero-indexed][17]**: the first element of an array is at index `0`, the second is at index `1`, and so on — and the last element is at the value of the array's [`length`][18] property minus `1`.
- **JavaScript [array-copy operations][19] create [shallow copies][20]**. (All standard built-in copy operations with *any* JavaScript objects create shallow copies, rather than [deep copies][21]).

#### [Array indices][22]

`Array` objects cannot use arbitrary strings as element indexes (as in an [associative array][23]) but must use nonnegative integers (or their respective string form). Setting or accessing via non-integers will not set or retrieve an element from the array list itself, but will set or access a variable associated with that array's [object property collection][24]. The array's object properties and list of array elements are separate, and the array's [traversal and mutation operations][25] cannot be applied to these named properties.

Array elements are object properties in the same way that `toString` is a property (to be specific, however, `toString()` is a method). Nevertheless, trying to access an element of an array as follows throws a syntax error because the property name is not valid:

js

```text
arr.0; // a syntax error
```

JavaScript syntax requires properties beginning with a digit to be accessed using [bracket notation][26] instead of [dot notation][27]. It's also possible to quote the array indices (e.g., `years['2']` instead of `years[2]`), although usually not necessary.

The `2` in `years[2]` is coerced into a string by the JavaScript engine through an implicit `toString` conversion. As a result, `'2'` and `'02'` would refer to two different slots on the `years` object, and the following example could be `true`:

js

```text
console.log(years["2"] !== years["02"]);
```

Only `years['2']` is an actual array index. `years['02']` is an arbitrary string property that will not be visited in array iteration.

#### [Relationship between length and numerical properties][28]

A JavaScript array's [`length`][18] property and numerical properties are connected.

Several of the built-in array methods (e.g., [`join()`][29], [`slice()`][30], [`indexOf()`][31], etc.) take into account the value of an array's [`length`][18] property when they're called.

Other methods (e.g., [`push()`][32], [`splice()`][33], etc.) also result in updates to an array's [`length`][18] property.

js

```text
const fruits = [];
fruits.push("banana", "apple", "peach");
console.log(fruits.length); // 3
```

When setting a property on a JavaScript array when the property is a valid array index and that index is outside the current bounds of the array, the engine will update the array's [`length`][18] property accordingly:

js

```text
fruits[5] = "mango";
console.log(fruits[5]); // 'mango'
console.log(Object.keys(fruits)); // ['0', '1', '2', '5']
console.log(fruits.length); // 6
```

Increasing the [`length`][18] extends the array by adding empty slots without creating any new elements — not even `undefined`.

js

```text
fruits.length = 10;
console.log(fruits); // ['banana', 'apple', 'peach', empty x 2, 'mango', empty x 4]
console.log(Object.keys(fruits)); // ['0', '1', '2', '5']
console.log(fruits.length); // 10
console.log(fruits[8]); // undefined
```

Decreasing the [`length`][18] property does, however, delete elements.

js

```text
fruits.length = 2;
console.log(Object.keys(fruits)); // ['0', '1']
console.log(fruits.length); // 2
```

This is explained further on the [`length`][18] page.

#### [Array methods and empty slots][34]

Array methods have different behaviors when encountering empty slots in [sparse arrays][35]. In general, older methods (e.g., `forEach`) treat empty slots differently from indices that contain `undefined`.

Methods that have special treatment for empty slots include the following: [`concat()`][36], [`copyWithin()`][37], [`every()`][38], [`filter()`][39], [`flat()`][40], [`flatMap()`][41], [`forEach()`][42], [`indexOf()`][31], [`lastIndexOf()`][43], [`map()`][44], [`reduce()`][45], [`reduceRight()`][46], [`reverse()`][47], [`slice()`][30], [`some()`][48], [`sort()`][49], and [`splice()`][33]. Iteration methods such as `forEach` don't visit empty slots at all. Other methods, such as `concat`, `copyWithin`, etc., preserve empty slots when doing the copying, so in the end the array is still sparse.

js

```text
const colors = ["red", "yellow", "blue"];
colors[5] = "purple";
colors.forEach((item, index) => {
  console.log(`${index}: ${item}`);
});
// Output:
// 0: red
// 1: yellow
// 2: blue
// 5: purple

colors.reverse(); // ['purple', empty × 2, 'blue', 'yellow', 'red']
```

Newer methods (e.g., `keys`) do not treat empty slots specially and treat them as if they contain `undefined`. Methods that conflate empty slots with `undefined` elements include the following: [`entries()`][50], [`fill()`][51], [`find()`][52], [`findIndex()`][53], [`findLast()`][54], [`findLastIndex()`][55], [`includes()`][56], [`join()`][29], [`keys()`][57], [`toLocaleString()`][58], [`toReversed()`][59], [`toSorted()`][60], [`toSpliced()`][61], [`values()`][62], and [`with()`][63].

js

```text
const colors = ["red", "yellow", "blue"];
colors[5] = "purple";
const iterator = colors.keys();
for (const key of iterator) {
  console.log(`${key}: ${colors[key]}`);
}
// Output
// 0: red
// 1: yellow
// 2: blue
// 3: undefined
// 4: undefined
// 5: purple

const newColors = colors.toReversed(); // ['purple', undefined, undefined, 'blue', 'yellow', 'red']
```

#### [Copying methods and mutating methods][64]

Some methods do not mutate the existing array that the method was called on, but instead return a new array. They do so by first constructing a new array and then populating it with elements. The copy always happens [*shallowly*][20] — the method never copies anything beyond the initially created array. Elements of the original array(s) are copied into the new array as follows:

- Objects: the object reference is copied into the new array. Both the original and new array refer to the same object. That is, if a referenced object is modified, the changes are visible to both the new and original arrays.
- Primitive types such as strings, numbers and booleans (not [`String`][65], [`Number`][66], and [`Boolean`][67] objects): their values are copied into the new array.


Other methods mutate the array that the method was called on, in which case their return value differs depending on the method: sometimes a reference to the same array, sometimes the length of the new array.

The following methods create new arrays by accessing [`this.constructor[Symbol.species]`][68] to determine the constructor to use: [`concat()`][36], [`filter()`][39], [`flat()`][40], [`flatMap()`][41], [`map()`][44], [`slice()`][30], and [`splice()`][33] (to construct the array of removed elements that's returned).

The following methods always create new arrays with the `Array` base constructor: [`toReversed()`][59], [`toSorted()`][60], [`toSpliced()`][61], and [`with()`][63].

The following table lists the methods that mutate the original array, and the corresponding non-mutating alternative:

| Mutating method         | Non-mutating alternative        |
| ----------------------- | ------------------------------- |
| [`copyWithin()`][37]    | No one-method alternative       |
| [`fill()`][51]          | No one-method alternative       |
| [`pop()`][69]           | [`slice(0, -1)`][30]            |
| [`push(v1, v2)`][32]    | [`concat([v1, v2])`][36]        |
| [`reverse()`][47]       | [`toReversed()`][59]            |
| [`shift()`][70]         | [`slice(1)`][30]                |
| [`sort()`][49]          | [`toSorted()`][60]              |
| [`splice()`][33]        | [`toSpliced()`][61]             |
| [`unshift(v1, v2)`][71] | [`toSpliced(0, 0, v1, v2)`][61] |

An easy way to change a mutating method into a non-mutating alternative is to use the [spread syntax][72] or [`slice()`][30] to create a copy first:

js

```text
arr.copyWithin(0, 1, 2); // mutates arr
const arr2 = arr.slice().copyWithin(0, 1, 2); // does not mutate arr
const arr3 = [...arr].copyWithin(0, 1, 2); // does not mutate arr
```

#### [Iterative methods][73]

Many array methods take a callback function as an argument. The callback function is called sequentially and at most once for each element in the array, and the return value of the callback function is used to determine the return value of the method. They all share the same signature:

js

```text
method(callbackFn, thisArg)
```

Where `callbackFn` takes three arguments:

[`element`][74]
The current element being processed in the array.

[`index`][75]
The index of the current element being processed in the array.

[`array`][76]
The array that the method was called upon.

What `callbackFn` is expected to return depends on the array method that was called.

The `thisArg` argument (defaults to `undefined`) will be used as the `this` value when calling `callbackFn`. The `this` value ultimately observable by `callbackFn` is determined according to [the usual rules][77]: if `callbackFn` is [non-strict][78], primitive `this` values are wrapped into objects, and `undefined`/`null` is substituted with [`globalThis`][79]. The `thisArg` argument is irrelevant for any `callbackFn` defined with an [arrow function][80], as arrow functions don't have their own `this` [binding][81].

The `array` argument passed to `callbackFn` is most useful if you want to read another index during iteration, because you may not always have an existing variable that refers to the current array. You should generally not mutate the array during iteration (see [mutating initial array in iterative methods][82]), but you can also use this argument to do so. The `array` argument is *not* the array that is being built, in the case of methods like `map()`, `filter()`, and `flatMap()` — there is no way to access the array being built from the callback function.

All iterative methods are [copying][64] and [generic][83], although they behave differently with [empty slots][34].

The following methods are iterative: [`every()`][38], [`filter()`][39], [`find()`][52], [`findIndex()`][53], [`findLast()`][54], [`findLastIndex()`][55], [`flatMap()`][41], [`forEach()`][42], [`map()`][44], and [`some()`][48].

In particular, [`every()`][38], [`find()`][52], [`findIndex()`][53], [`findLast()`][54], [`findLastIndex()`][55], and [`some()`][48] do not always invoke `callbackFn` on every element — they stop iteration as soon as the return value is determined.

The [`reduce()`][45] and [`reduceRight()`][46] methods also take a callback function and run it at most once for each element in the array, but they have slightly different signatures from typical iterative methods (for example, they don't accept `thisArg`).

The [`sort()`][49] method also takes a callback function, but it is not an iterative method. It mutates the array in-place, doesn't accept `thisArg`, and may invoke the callback multiple times on an index.

Iterative methods iterate the array like the following (with a lot of technical details omitted):

js

```text
function method(callbackFn, thisArg) {
  const length = this.length;
  for (let i = 0; i < length; i++) {
    if (i in this) {
      const result = callbackFn.call(thisArg, this[i], i, this);
      // Do something with result; maybe return early
    }
  }
}
```

Note the following:

1. Not all methods do the `i in this` test. The `find`, `findIndex`, `findLast`, and `findLastIndex` methods do not, but other methods do.
2. The `length` is memorized before the loop starts. This affects how insertions and deletions during iteration are handled (see [mutating initial array in iterative methods][82]).
3. The method doesn't memorize the array contents, so if any index is modified during iteration, the new value might be observed.
4. The code above iterates the array in ascending order of index. Some methods iterate in descending order of index (`for (let i = length - 1; i >= 0; i--)`): `reduceRight()`, `findLast()`, and `findLastIndex()`.
5. `reduce` and `reduceRight` have slightly different signatures and do not always start at the first/last element.

#### [Generic array methods][83]

Array methods are always generic — they don't access any internal data of the array object. They only access the array elements through the `length` property and the indexed elements. This means that they can be called on array-like objects as well.

js

```text
const arrayLike = {
  0: "a",
  1: "b",
  length: 2,
};
console.log(Array.prototype.join.call(arrayLike, "+")); // 'a+b'
```

##### Normalization of the length property

The `length` property is [converted to an integer][84] and then clamped to the range between 0 and 253 - 1. `NaN` becomes `0`, so even when `length` is not present or is `undefined`, it behaves as if it has value `0`.

The language avoids setting `length` to an [unsafe integer][85]. All built-in methods will throw a [`TypeError`][86] if `length` will be set to a number greater than 253 - 1. However, because the [`length`][18] property of arrays throws an error if it's set to greater than 232 - 1, the safe integer threshold is usually not reached unless the method is called on a non-array object.

js

```text
Array.prototype.flat.call({}); // []
```

Some array methods set the `length` property of the array object. They always set the value after normalization, so `length` always ends as an integer.

js

```text
const a = { length: 0.7 };
Array.prototype.push.call(a);
console.log(a.length); // 0
```

##### Array-like objects

The term [*array-like object*][87] refers to any object that doesn't throw during the `length` conversion process described above. In practice, such object is expected to actually have a `length` property and to have indexed elements in the range `0` to `length - 1`. (If it doesn't have all indices, it will be functionally equivalent to a [sparse array][34].) Any integer index less than zero or greater than `length - 1` is ignored when an array method operates on an array-like object.

Many DOM objects are array-like — for example, [`NodeList`][88] and [`HTMLCollection`][89]. The [`arguments`][90] object is also array-like. You can call array methods on them even if they don't have these methods themselves.

js

```text
function f() {
  console.log(Array.prototype.join.call(arguments, "+"));
}

f("a", "b"); // 'a+b'
```

### [Constructor][91]

[`Array()`][92]
Creates a new `Array` object.

### [Static properties][93]

[`Array[Symbol.species]`][68]
Returns the `Array` constructor.

### [Static methods][94]

[`Array.from()`][95]
Creates a new `Array` instance from an iterable or array-like object.

[`Array.fromAsync()`][96]
Creates a new `Array` instance from an async iterable, iterable, or array-like object.

[`Array.isArray()`][97]
Returns `true` if the argument is an array, or `false` otherwise.

[`Array.of()`][98]
Creates a new `Array` instance with a variable number of arguments, regardless of number or type of the arguments.

### [Instance properties][99]

These properties are defined on `Array.prototype` and shared by all `Array` instances.

[`Array.prototype.constructor`][100]
The constructor function that created the instance object. For `Array` instances, the initial value is the [`Array`][92] constructor.

[`Array.prototype[Symbol.unscopables]`][101]
Contains property names that were not included in the ECMAScript standard prior to the ES2015 version and that are ignored for [`with`][102] statement-binding purposes.

These properties are own properties of each `Array` instance.

[`length`][18]
Reflects the number of elements in an array.

### [Instance methods][103]

[`Array.prototype.at()`][104]
Returns the array item at the given index. Accepts negative integers, which count back from the last item.

[`Array.prototype.concat()`][36]
Returns a new array that is the calling array joined with other array(s) and/or value(s).

[`Array.prototype.copyWithin()`][37]
Copies a sequence of array elements within an array.

[`Array.prototype.entries()`][50]
Returns a new [*array iterator*][105] object that contains the key/value pairs for each index in an array.

[`Array.prototype.every()`][38]
Returns `false` if it finds an element in the array that does not satisfy the provided testing function. Otherwise, it returns `true`.

[`Array.prototype.fill()`][51]
Fills all the elements of an array from a start index to an end index with a static value.

[`Array.prototype.filter()`][39]
Returns a new array containing all elements of the calling array for which the provided filtering function returns `true`.

[`Array.prototype.find()`][52]
Returns the value of the first element in the array that satisfies the provided testing function, or `undefined` if no appropriate element is found.

[`Array.prototype.findIndex()`][53]
Returns the index of the first element in the array that satisfies the provided testing function, or `-1` if no appropriate element was found.

[`Array.prototype.findLast()`][54]
Returns the value of the last element in the array that satisfies the provided testing function, or `undefined` if no appropriate element is found.

[`Array.prototype.findLastIndex()`][55]
Returns the index of the last element in the array that satisfies the provided testing function, or `-1` if no appropriate element was found.

[`Array.prototype.flat()`][40]
Returns a new array with all sub-array elements concatenated into it recursively up to the specified depth.

[`Array.prototype.flatMap()`][41]
Returns a new array formed by applying a given callback function to each element of the calling array, and then flattening the result by one level.

[`Array.prototype.forEach()`][42]
Calls a function for each element in the calling array.

[`Array.prototype.includes()`][56]
Determines whether the calling array contains a value, returning `true` or `false` as appropriate.

[`Array.prototype.indexOf()`][31]
Returns the first (least) index at which a given element can be found in the calling array.

[`Array.prototype.join()`][29]
Joins all elements of an array into a string.

[`Array.prototype.keys()`][57]
Returns a new [*array iterator*][105] that contains the keys for each index in the calling array.

[`Array.prototype.lastIndexOf()`][43]
Returns the last (greatest) index at which a given element can be found in the calling array, or `-1` if none is found.

[`Array.prototype.map()`][44]
Returns a new array containing the results of invoking a function on every element in the calling array.

[`Array.prototype.pop()`][69]
Removes the last element from an array and returns that element.

[`Array.prototype.push()`][32]
Adds one or more elements to the end of an array, and returns the new `length` of the array.

[`Array.prototype.reduce()`][45]
Executes a user-supplied "reducer" callback function on each element of the array (from left to right), to reduce it to a single value.

[`Array.prototype.reduceRight()`][46]
Executes a user-supplied "reducer" callback function on each element of the array (from right to left), to reduce it to a single value.

[`Array.prototype.reverse()`][47]
Reverses the order of the elements of an array *in place*. (First becomes the last, last becomes first.)

[`Array.prototype.shift()`][70]
Removes the first element from an array and returns that element.

[`Array.prototype.slice()`][30]
Extracts a section of the calling array and returns a new array.

[`Array.prototype.some()`][48]
Returns `true` if it finds an element in the array that satisfies the provided testing function. Otherwise, it returns `false`.

[`Array.prototype.sort()`][49]
Sorts the elements of an array in place and returns the array.

[`Array.prototype.splice()`][33]
Adds and/or removes elements from an array.

[`Array.prototype.toLocaleString()`][58]
Returns a localized string representing the calling array and its elements. Overrides the [`Object.prototype.toLocaleString()`][106] method.

[`Array.prototype.toReversed()`][59]
Returns a new array with the elements in reversed order, without modifying the original array.

[`Array.prototype.toSorted()`][60]
Returns a new array with the elements sorted in ascending order, without modifying the original array.

[`Array.prototype.toSpliced()`][61]
Returns a new array with some elements removed and/or replaced at a given index, without modifying the original array.

[`Array.prototype.toString()`][107]
Returns a string representing the calling array and its elements. Overrides the [`Object.prototype.toString()`][108] method.

[`Array.prototype.unshift()`][71]
Adds one or more elements to the front of an array, and returns the new `length` of the array.

[`Array.prototype.values()`][62]
Returns a new [*array iterator*][105] object that contains the values for each index in the array.

[`Array.prototype.with()`][63]
Returns a new array with the element at the given index replaced with the given value, without modifying the original array.

[`Array.prototype[Symbol.iterator]()`][109]
An alias for the [`values()`][62] method by default.

### [Examples][12]

This section provides some examples of common array operations in JavaScript.

**Note:** If you're not yet familiar with array basics, consider first reading [JavaScript First Steps: Arrays][11], which [explains what arrays are][110], and includes other examples of common array operations.

#### [Create an array][111]

This example shows three ways to create new array: first using [array literal notation][112], then using the [`Array()`][92] constructor, and finally using [`String.prototype.split()`][113] to build the array from a string.

js

```text
// 'fruits' array created using array literal notation.
const fruits = ["Apple", "Banana"];
console.log(fruits.length);
// 2

// 'fruits2' array created using the Array() constructor.
const fruits2 = new Array("Apple", "Banana");
console.log(fruits2.length);
// 2

// 'fruits3' array created using String.prototype.split().
const fruits3 = "Apple, Banana".split(", ");
console.log(fruits3.length);
// 2
```

#### [Create a string from an array][114]

This example uses the [`join()`][29] method to create a string from the `fruits` array.

js

```text
const fruits = ["Apple", "Banana"];
const fruitsString = fruits.join(", ");
console.log(fruitsString);
// "Apple, Banana"
```

#### [Access an array item by its index][115]

This example shows how to access items in the `fruits` array by specifying the index number of their position in the array.

js

```text
const fruits = ["Apple", "Banana"];

// The index of an array's first element is always 0.
fruits[0]; // Apple

// The index of an array's second element is always 1.
fruits[1]; // Banana

// The index of an array's last element is always one
// less than the length of the array.
fruits[fruits.length - 1]; // Banana

// Using an index number larger than the array's length
// returns 'undefined'.
fruits[99]; // undefined
```

#### [Find the index of an item in an array][116]

This example uses the [`indexOf()`][31] method to find the position (index) of the string `"Banana"` in the `fruits` array.

js

```text
const fruits = ["Apple", "Banana"];
console.log(fruits.indexOf("Banana"));
// 1
```

#### [Check if an array contains a certain item][117]

This example shows two ways to check if the `fruits` array contains `"Banana"` and `"Cherry"`: first with the [`includes()`][56] method, and then with the [`indexOf()`][31] method to test for an index value that's not `-1`.

js

```text
const fruits = ["Apple", "Banana"];

fruits.includes("Banana"); // true
fruits.includes("Cherry"); // false

// If indexOf() doesn't return -1, the array contains the given item.
fruits.indexOf("Banana") !== -1; // true
fruits.indexOf("Cherry") !== -1; // false
```

#### [Append an item to an array][118]

This example uses the [`push()`][32] method to append a new string to the `fruits` array.

js

```text
const fruits = ["Apple", "Banana"];
const newLength = fruits.push("Orange");
console.log(fruits);
// ["Apple", "Banana", "Orange"]
console.log(newLength);
// 3
```

#### [Remove the last item from an array][119]

This example uses the [`pop()`][69] method to remove the last item from the `fruits` array.

js

```text
const fruits = ["Apple", "Banana", "Orange"];
const removedItem = fruits.pop();
console.log(fruits);
// ["Apple", "Banana"]
console.log(removedItem);
// Orange
```

**Note:** `pop()` can only be used to remove the last item from an array. To remove multiple items from the end of an array, see the next example.

#### [Remove multiple items from the end of an array][120]

This example uses the [`splice()`][33] method to remove the last 3 items from the `fruits` array.

js

```text
const fruits = ["Apple", "Banana", "Strawberry", "Mango", "Cherry"];
const start = -3;
const removedItems = fruits.splice(start);
console.log(fruits);
// ["Apple", "Banana"]
console.log(removedItems);
// ["Strawberry", "Mango", "Cherry"]
```

#### [Truncate an array down to just its first N items][121]

This example uses the [`splice()`][33] method to truncate the `fruits` array down to just its first 2 items.

js

```text
const fruits = ["Apple", "Banana", "Strawberry", "Mango", "Cherry"];
const start = 2;
const removedItems = fruits.splice(start);
console.log(fruits);
// ["Apple", "Banana"]
console.log(removedItems);
// ["Strawberry", "Mango", "Cherry"]
```

#### [Remove the first item from an array][122]

This example uses the [`shift()`][70] method to remove the first item from the `fruits` array.

js

```text
const fruits = ["Apple", "Banana"];
const removedItem = fruits.shift();
console.log(fruits);
// ["Banana"]
console.log(removedItem);
// Apple
```

**Note:** `shift()` can only be used to remove the first item from an array. To remove multiple items from the beginning of an array, see the next example.

#### [Remove multiple items from the beginning of an array][123]

This example uses the [`splice()`][33] method to remove the first 3 items from the `fruits` array.

js

```text
const fruits = ["Apple", "Strawberry", "Cherry", "Banana", "Mango"];
const start = 0;
const deleteCount = 3;
const removedItems = fruits.splice(start, deleteCount);
console.log(fruits);
// ["Banana", "Mango"]
console.log(removedItems);
// ["Apple", "Strawberry", "Cherry"]
```

#### [Add a new first item to an array][124]

This example uses the [`unshift()`][71] method to add, at index `0`, a new item to the `fruits` array — making it the new first item in the array.

js

```text
const fruits = ["Banana", "Mango"];
const newLength = fruits.unshift("Strawberry");
console.log(fruits);
// ["Strawberry", "Banana", "Mango"]
console.log(newLength);
// 3
```

#### [Remove a single item by index][125]

This example uses the [`splice()`][33] method to remove the string `"Banana"` from the `fruits` array — by specifying the index position of `"Banana"`.

js

```text
const fruits = ["Strawberry", "Banana", "Mango"];
const start = fruits.indexOf("Banana");
const deleteCount = 1;
const removedItems = fruits.splice(start, deleteCount);
console.log(fruits);
// ["Strawberry", "Mango"]
console.log(removedItems);
// ["Banana"]
```

#### [Remove multiple items by index][126]

This example uses the [`splice()`][33] method to remove the strings `"Banana"` and `"Strawberry"` from the `fruits` array — by specifying the index position of `"Banana"`, along with a count of the number of total items to remove.

js

```text
const fruits = ["Apple", "Banana", "Strawberry", "Mango"];
const start = 1;
const deleteCount = 2;
const removedItems = fruits.splice(start, deleteCount);
console.log(fruits);
// ["Apple", "Mango"]
console.log(removedItems);
// ["Banana", "Strawberry"]
```

#### [Replace multiple items in an array][127]

This example uses the [`splice()`][33] method to replace the last 2 items in the `fruits` array with new items.

js

```text
const fruits = ["Apple", "Banana", "Strawberry"];
const start = -2;
const deleteCount = 2;
const removedItems = fruits.splice(start, deleteCount, "Mango", "Cherry");
console.log(fruits);
// ["Apple", "Mango", "Cherry"]
console.log(removedItems);
// ["Banana", "Strawberry"]
```

#### [Iterate over an array][128]

This example uses a [`for...of`][129] loop to iterate over the `fruits` array, logging each item to the console.

js

```text
const fruits = ["Apple", "Mango", "Cherry"];
for (const fruit of fruits) {
  console.log(fruit);
}
// Apple
// Mango
// Cherry
```

But `for...of` is just one of many ways to iterate over any array; for more ways, see [Loops and iteration][130], and see the documentation for the [`every()`][38], [`filter()`][39], [`flatMap()`][41], [`map()`][44], [`reduce()`][45], and [`reduceRight()`][46] methods — and see the next example, which uses the [`forEach()`][42] method.

#### [Call a function on each element in an array][131]

This example uses the [`forEach()`][42] method to call a function on each element in the `fruits` array; the function causes each item to be logged to the console, along with the item's index number.

js

```text
const fruits = ["Apple", "Mango", "Cherry"];
fruits.forEach((item, index, array) => {
  console.log(item, index);
});
// Apple 0
// Mango 1
// Cherry 2
```

#### [Merge multiple arrays together][132]

This example uses the [`concat()`][36] method to merge the `fruits` array with a `moreFruits` array, to produce a new `combinedFruits` array. Notice that `fruits` and `moreFruits` remain unchanged.

js

```text
const fruits = ["Apple", "Banana", "Strawberry"];
const moreFruits = ["Mango", "Cherry"];
const combinedFruits = fruits.concat(moreFruits);
console.log(combinedFruits);
// ["Apple", "Banana", "Strawberry", "Mango", "Cherry"]

// The 'fruits' array remains unchanged.
console.log(fruits);
// ["Apple", "Banana", "Strawberry"]

// The 'moreFruits' array also remains unchanged.
console.log(moreFruits);
// ["Mango", "Cherry"]
```

#### [Copy an array][19]

This example shows three ways to create a new array from the existing `fruits` array: first by using [spread syntax][72], then by using the [`from()`][95] method, and then by using the [`slice()`][30] method.

js

```text
const fruits = ["Strawberry", "Mango"];

// Create a copy using spread syntax.
const fruitsCopy = [...fruits];
// ["Strawberry", "Mango"]

// Create a copy using the from() method.
const fruitsCopy2 = Array.from(fruits);
// ["Strawberry", "Mango"]

// Create a copy using the slice() method.
const fruitsCopy3 = fruits.slice();
// ["Strawberry", "Mango"]
```

All built-in array-copy operations ([spread syntax][72], [`Array.from()`][95], [`Array.prototype.slice()`][30], and [`Array.prototype.concat()`][36]) create [shallow copies][20]. If you instead want a [deep copy][21] of an array, you can use [`JSON.stringify()`][133] to convert the array to a JSON string, and then [`JSON.parse()`][134] to convert the string back into a new array that's completely independent from the original array.

js

```text
const fruitsDeepCopy = JSON.parse(JSON.stringify(fruits));
```

You can also create deep copies using the [`structuredClone()`][135] method, which has the advantage of allowing [transferable objects][136] in the source to be *transferred* to the new copy, rather than just cloned.

Finally, it's important to understand that assigning an existing array to a new variable doesn't create a copy of either the array or its elements. Instead the new variable is just a reference, or alias, to the original array; that is, the original array's name and the new variable name are just two names for the exact same object (and so will always evaluate as [strictly equivalent][137]). Therefore, if you make any changes at all either to the value of the original array or to the value of the new variable, the other will change, too:

js

```text
const fruits = ["Strawberry", "Mango"];
const fruitsAlias = fruits;
// 'fruits' and 'fruitsAlias' are the same object, strictly equivalent.
fruits === fruitsAlias; // true
// Any changes to the 'fruits' array change 'fruitsAlias' too.
fruits.unshift("Apple", "Banana");
console.log(fruits);
// ['Apple', 'Banana', 'Strawberry', 'Mango']
console.log(fruitsAlias);
// ['Apple', 'Banana', 'Strawberry', 'Mango']
```

#### [Creating a two-dimensional array][138]

The following creates a chessboard as a two-dimensional array of strings. The first move is made by copying the `'p'` in `board[6][4]` to `board[4][4]`. The old position at `[6][4]` is made blank.

js

```text
const board = [
  ["R", "N", "B", "Q", "K", "B", "N", "R"],
  ["P", "P", "P", "P", "P", "P", "P", "P"],
  [" ", " ", " ", " ", " ", " ", " ", " "],
  [" ", " ", " ", " ", " ", " ", " ", " "],
  [" ", " ", " ", " ", " ", " ", " ", " "],
  [" ", " ", " ", " ", " ", " ", " ", " "],
  ["p", "p", "p", "p", "p", "p", "p", "p"],
  ["r", "n", "b", "q", "k", "b", "n", "r"],
];

console.log(`${board.join("\n")}\n\n`);

// Move King's Pawn forward 2
board[4][4] = board[6][4];
board[6][4] = " ";
console.log(board.join("\n"));
```

Here is the output:

```text
R,N,B,Q,K,B,N,R
P,P,P,P,P,P,P,P
 , , , , , , ,
 , , , , , , ,
 , , , , , , ,
 , , , , , , ,
p,p,p,p,p,p,p,p
r,n,b,q,k,b,n,r

R,N,B,Q,K,B,N,R
P,P,P,P,P,P,P,P
 , , , , , , ,
 , , , , , , ,
 , , , ,p, , ,
 , , , , , , ,
p,p,p,p, ,p,p,p
r,n,b,q,k,b,n,r
```

#### [Using an array to tabulate a set of values][139]

js

```text
const values = [];
for (let x = 0; x < 10; x++) {
  values.push([2 ** x, 2 * x ** 2]);
}
console.table(values);
```

Results in

```text
// The first column is the index
0  1    0
1  2    2
2  4    8
3  8    18
4  16   32
5  32   50
6  64   72
7  128  98
8  256  128
9  512  162
```

#### [Creating an array using the result of a match][140]

The result of a match between a [`RegExp`][141] and a string can create a JavaScript array that has properties and elements which provide information about the match. Such an array is returned by [`RegExp.prototype.exec()`][142] and [`String.prototype.match()`][143].

For example:

js

```text
// Match one d followed by one or more b's followed by one d
// Remember matched b's and the following d
// Ignore case

const myRe = /d(b+)(d)/i;
const execResult = myRe.exec("cdbBdbsbz");

console.log(execResult.input); // 'cdbBdbsbz'
console.log(execResult.index); // 1
console.log(execResult); // [ "dbBd", "bB", "d" ]
```

For more information about the result of a match, see the [`RegExp.prototype.exec()`][142] and [`String.prototype.match()`][143] pages.

#### [Mutating initial array in iterative methods][82]

[Iterative methods][73] do not mutate the array on which it is called, but the function provided as `callbackFn` can. The key principle to remember is that only indexes between 0 and `arrayLength - 1` are visited, where `arrayLength` is the length of the array at the time the array method was first called, but the element passed to the callback is the value at the time the index is visited. Therefore:

- `callbackFn` will not visit any elements added beyond the array's initial length when the call to the iterative method began.
- Changes to already-visited indexes do not cause `callbackFn` to be invoked on them again.
- If an existing, yet-unvisited element of the array is changed by `callbackFn`, its value passed to the `callbackFn` will be the value at the time that element gets visited. Removed elements are not visited.


**Warning:** Concurrent modifications of the kind described above frequently lead to hard-to-understand code and are generally to be avoided (except in special cases).

The following examples use the `forEach` method as an example, but other methods that visit indexes in ascending order work in the same way. We will first define a helper function:

js

```text
function testSideEffect(effect) {
  const arr = ["e1", "e2", "e3", "e4"];
  arr.forEach((elem, index, arr) => {
    console.log(`array: [${arr.join(", ")}], index: ${index}, elem: ${elem}`);
    effect(arr, index);
  });
  console.log(`Final array: [${arr.join(", ")}]`);
}
```

Modification to indexes not visited yet will be visible once the index is reached:

js

```text
testSideEffect((arr, index) => {
  if (index + 1 < arr.length) arr[index + 1] += "*";
});
// array: [e1, e2, e3, e4], index: 0, elem: e1
// array: [e1, e2*, e3, e4], index: 1, elem: e2*
// array: [e1, e2*, e3*, e4], index: 2, elem: e3*
// array: [e1, e2*, e3*, e4*], index: 3, elem: e4*
// Final array: [e1, e2*, e3*, e4*]
```

Modification to already visited indexes does not change iteration behavior, although the array will be different afterwards:

js

```text
testSideEffect((arr, index) => {
  if (index > 0) arr[index - 1] += "*";
});
// array: [e1, e2, e3, e4], index: 0, elem: e1
// array: [e1, e2, e3, e4], index: 1, elem: e2
// array: [e1*, e2, e3, e4], index: 2, elem: e3
// array: [e1*, e2*, e3, e4], index: 3, elem: e4
// Final array: [e1*, e2*, e3*, e4]
```

Inserting *n* elements at unvisited indexes that are less than the initial array length will make them be visited. The last *n* elements in the original array that now have index greater than the initial array length will not be visited:

js

```text
testSideEffect((arr, index) => {
  if (index === 1) arr.splice(2, 0, "new");
});
// array: [e1, e2, e3, e4], index: 0, elem: e1
// array: [e1, e2, e3, e4], index: 1, elem: e2
// array: [e1, e2, new, e3, e4], index: 2, elem: new
// array: [e1, e2, new, e3, e4], index: 3, elem: e3
// Final array: [e1, e2, new, e3, e4]
// e4 is not visited because it now has index 4
```

Inserting *n* elements with index greater than the initial array length will not make them be visited:

js

```text
testSideEffect((arr) => arr.push("new"));
// array: [e1, e2, e3, e4], index: 0, elem: e1
// array: [e1, e2, e3, e4, new], index: 1, elem: e2
// array: [e1, e2, e3, e4, new, new], index: 2, elem: e3
// array: [e1, e2, e3, e4, new, new, new], index: 3, elem: e4
// Final array: [e1, e2, e3, e4, new, new, new, new]
```

Inserting *n* elements at already visited indexes will not make them be visited, but it shifts remaining elements back by *n*, so the current index and the *n - 1* elements before it are visited again:

js

```text
testSideEffect((arr, index) => arr.splice(index, 0, "new"));
// array: [e1, e2, e3, e4], index: 0, elem: e1
// array: [new, e1, e2, e3, e4], index: 1, elem: e1
// array: [new, new, e1, e2, e3, e4], index: 2, elem: e1
// array: [new, new, new, e1, e2, e3, e4], index: 3, elem: e1
// Final array: [new, new, new, new, e1, e2, e3, e4]
// e1 keeps getting visited because it keeps getting shifted back
```

Deleting *n* elements at unvisited indexes will make them not be visited anymore. Because the array has shrunk, the last *n* iterations will visit out-of-bounds indexes. If the method ignores non-existent indexes (see [array methods and empty slots][34]), the last *n* iterations will be skipped; otherwise, they will receive `undefined`:

js

```text
testSideEffect((arr, index) => {
  if (index === 1) arr.splice(2, 1);
});
// array: [e1, e2, e3, e4], index: 0, elem: e1
// array: [e1, e2, e3, e4], index: 1, elem: e2
// array: [e1, e2, e4], index: 2, elem: e4
// Final array: [e1, e2, e4]
// Does not visit index 3 because it's out-of-bounds

// Compare this with find(), which treats nonexistent indexes as undefined:
const arr2 = ["e1", "e2", "e3", "e4"];
arr2.find((elem, index, arr) => {
  console.log(`array: [${arr.join(", ")}], index: ${index}, elem: ${elem}`);
  if (index === 1) arr.splice(2, 1);
  return false;
});
// array: [e1, e2, e3, e4], index: 0, elem: e1
// array: [e1, e2, e3, e4], index: 1, elem: e2
// array: [e1, e2, e4], index: 2, elem: e4
// array: [e1, e2, e4], index: 3, elem: undefined
```

Deleting *n* elements at already visited indexes does not change the fact that they were visited before they get deleted. Because the array has shrunk, the next *n* elements after the current index are skipped. If the method ignores non-existent indexes, the last *n* iterations will be skipped; otherwise, they will receive `undefined`:

js

```text
testSideEffect((arr, index) => arr.splice(index, 1));
// array: [e1, e2, e3, e4], index: 0, elem: e1
// Does not visit e2 because e2 now has index 0, which has already been visited
// array: [e2, e3, e4], index: 1, elem: e3
// Does not visit e4 because e4 now has index 1, which has already been visited
// Final array: [e2, e4]
// Index 2 is out-of-bounds, so it's not visited

// Compare this with find(), which treats nonexistent indexes as undefined:
const arr2 = ["e1", "e2", "e3", "e4"];
arr2.find((elem, index, arr) => {
  console.log(`array: [${arr.join(", ")}], index: ${index}, elem: ${elem}`);
  arr.splice(index, 1);
  return false;
});
// array: [e1, e2, e3, e4], index: 0, elem: e1
// array: [e2, e3, e4], index: 1, elem: e3
// array: [e2, e4], index: 2, elem: undefined
// array: [e2, e4], index: 3, elem: undefined
```

For methods that iterate in descending order of index, insertion causes elements to be skipped, and deletion causes elements to be visited multiple times. Adjust the code above yourself to see the effects.

### [Specifications][144]

| Specification                                                      |
| ------------------------------------------------------------------ |
| [ECMAScript® 2026 Language Specification # sec-array-objects][145] |

### [Browser compatibility][9]

### [See also][146]

- [Indexed collections][147] guide
- [`TypedArray`][148]
- [`ArrayBuffer`][149]

### Help improve MDN

[Learn how to contribute][150]

This page was last modified on Feb 24, 2026 by [MDN contributors][151].



[View this page on GitHub][152] • [Report a problem with this content][153]

[/][154]

Your blueprint for a better internet.

- [][155]
- [][156]
- [][157]
- [][158]
- [][159]


MDN

- [About][160]
- [Blog][161]
- [Mozilla careers][162]
- [Advertise with us][163]
- [MDN Plus][164]
- [Product help][165]

Contribute

- [MDN Community][166]
- [Community resources][167]
- [Writing guidelines][168]
- [MDN Discord][169]
- [MDN on GitHub][170]

Developers

- [Web technologies][3]
- [Learn web development][171]
- [Guides][172]
- [Tutorials][173]
- [Glossary][174]
- [Hacks blog][175]

[https://www.mozilla.org/][176]

- [Website Privacy Notice][177]
- [Telemetry Settings][178]
- [Legal][179]
- [Community Participation Guidelines][180]


Visit [Mozilla Corporation’s][176] not-for-profit parent, the [Mozilla Foundation][181].
Portions of this content are ©1998–2026 by individual mozilla.org contributors. Content available under [a Creative Commons license][182].

[1]: #content
[2]: #search
[3]: /en-US/docs/Web
[4]: /en-US/docs/Web/JavaScript
[5]: /en-US/docs/Web/JavaScript/Reference
[6]: /en-US/docs/Web/JavaScript/Reference/Global_Objects
[7]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array
[8]: /en-US/docs/Glossary/Baseline/Compatibility
[9]: #browser_compatibility
[10]: https://survey.alchemer.com/s3/7634825/MDN-baseline-feedback?page=%2Fen-US%2Fdocs%2FWeb%2FJavaScript%2FReference%2FGlobal_Objects%2FArray&level=high
[11]: /en-US/docs/Learn_web_development/Core/Scripting/Arrays
[12]: #examples
[13]: #description
[14]: /en-US/docs/Glossary/Primitive
[15]: /en-US/docs/Web/JavaScript/Guide/Data_structures
[16]: /en-US/docs/Web/JavaScript/Guide/Typed_arrays
[17]: https://en.wikipedia.org/wiki/Zero-based_numbering
[18]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/length
[19]: #copy_an_array
[20]: /en-US/docs/Glossary/Shallow_copy
[21]: /en-US/docs/Glossary/Deep_copy
[22]: #array_indices
[23]: https://en.wikipedia.org/wiki/Associative_array
[24]: /en-US/docs/Web/JavaScript/Guide/Data_structures#properties
[25]: /en-US/docs/Web/JavaScript/Guide/Indexed_collections#array_methods
[26]: /en-US/docs/Web/JavaScript/Guide/Working_with_objects#objects_and_properties
[27]: /en-US/docs/Web/JavaScript/Reference/Operators/Property_accessors
[28]: #relationship_between_length_and_numerical_properties
[29]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/join
[30]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/slice
[31]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/indexOf
[32]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/push
[33]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/splice
[34]: #array_methods_and_empty_slots
[35]: /en-US/docs/Web/JavaScript/Guide/Indexed_collections#sparse_arrays
[36]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/concat
[37]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/copyWithin
[38]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/every
[39]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/filter
[40]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/flat
[41]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/flatMap
[42]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/forEach
[43]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/lastIndexOf
[44]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/map
[45]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/reduce
[46]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/reduceRight
[47]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/reverse
[48]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/some
[49]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/sort
[50]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/entries
[51]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/fill
[52]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/find
[53]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/findIndex
[54]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/findLast
[55]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/findLastIndex
[56]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/includes
[57]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/keys
[58]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/toLocaleString
[59]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/toReversed
[60]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/toSorted
[61]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/toSpliced
[62]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/values
[63]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/with
[64]: #copying_methods_and_mutating_methods
[65]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/String
[66]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Number
[67]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Boolean
[68]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/Symbol.species
[69]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/pop
[70]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/shift
[71]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/unshift
[72]: /en-US/docs/Web/JavaScript/Reference/Operators/Spread_syntax
[73]: #iterative_methods
[74]: #element
[75]: #index
[76]: #array
[77]: /en-US/docs/Web/JavaScript/Reference/Operators/this
[78]: /en-US/docs/Web/JavaScript/Reference/Strict_mode#no_this_substitution
[79]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/globalThis
[80]: /en-US/docs/Web/JavaScript/Reference/Functions/Arrow_functions
[81]: /en-US/docs/Glossary/Binding
[82]: #mutating_initial_array_in_iterative_methods
[83]: #generic_array_methods
[84]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Number#integer_conversion
[85]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/MAX_SAFE_INTEGER
[86]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/TypeError
[87]: /en-US/docs/Web/JavaScript/Guide/Indexed_collections#working_with_array-like_objects
[88]: /en-US/docs/Web/API/NodeList
[89]: /en-US/docs/Web/API/HTMLCollection
[90]: /en-US/docs/Web/JavaScript/Reference/Functions/arguments
[91]: #constructor
[92]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/Array
[93]: #static_properties
[94]: #static_methods
[95]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/from
[96]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/fromAsync
[97]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/isArray
[98]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/of
[99]: #instance_properties
[100]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/constructor
[101]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/Symbol.unscopables
[102]: /en-US/docs/Web/JavaScript/Reference/Statements/with
[103]: #instance_methods
[104]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/at
[105]: /en-US/docs/Web/JavaScript/Guide/Iterators_and_generators
[106]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/toLocaleString
[107]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/toString
[108]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/toString
[109]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/Symbol.iterator
[110]: /en-US/docs/Learn_web_development/Core/Scripting/Arrays#what_is_an_array
[111]: #create_an_array
[112]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/Array#array_literal_notation
[113]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/String/split
[114]: #create_a_string_from_an_array
[115]: #access_an_array_item_by_its_index
[116]: #find_the_index_of_an_item_in_an_array
[117]: #check_if_an_array_contains_a_certain_item
[118]: #append_an_item_to_an_array
[119]: #remove_the_last_item_from_an_array
[120]: #remove_multiple_items_from_the_end_of_an_array
[121]: #truncate_an_array_down_to_just_its_first_n_items
[122]: #remove_the_first_item_from_an_array
[123]: #remove_multiple_items_from_the_beginning_of_an_array
[124]: #add_a_new_first_item_to_an_array
[125]: #remove_a_single_item_by_index
[126]: #remove_multiple_items_by_index
[127]: #replace_multiple_items_in_an_array
[128]: #iterate_over_an_array
[129]: /en-US/docs/Web/JavaScript/Reference/Statements/for...of
[130]: /en-US/docs/Web/JavaScript/Guide/Loops_and_iteration
[131]: #call_a_function_on_each_element_in_an_array
[132]: #merge_multiple_arrays_together
[133]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/JSON/stringify
[134]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/JSON/parse
[135]: /en-US/docs/Web/API/Window/structuredClone "structuredClone()"
[136]: /en-US/docs/Web/API/Web_Workers_API/Transferable_objects
[137]: /en-US/docs/Web/JavaScript/Guide/Equality_comparisons_and_sameness#strict_equality_using
[138]: #creating_a_two-dimensional_array
[139]: #using_an_array_to_tabulate_a_set_of_values
[140]: #creating_an_array_using_the_result_of_a_match
[141]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp
[142]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/exec
[143]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/String/match
[144]: #specifications
[145]: https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array-objects
[146]: #see_also
[147]: /en-US/docs/Web/JavaScript/Guide/Indexed_collections
[148]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray
[149]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer
[150]: /en-US/docs/MDN/Community/Getting_started
[151]: /en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/contributors.txt
[152]: https://github.com/mdn/content/blob/main/files/en-us/web/javascript/reference/global_objects/array/index.md?plain=1 "Folder: en-us/web/javascript/reference/global_objects/array (Opens in a new tab)"
[153]: https://github.com/mdn/content/issues/new?template=page-report.yml&mdn-url=https%3A%2F%2Fdeveloper.mozilla.org%2Fen-US%2Fdocs%2FWeb%2FJavaScript%2FReference%2FGlobal_Objects%2FArray&metadata=%3C%21--+Do+not+make+changes+below+this+line+--%3E%0A%3Cdetails%3E%0A%3Csummary%3EPage+report+details%3C%2Fsummary%3E%0A%0A*+Folder%3A+%60en-us%2Fweb%2Fjavascript%2Freference%2Fglobal_objects%2Farray%60%0A*+MDN+URL%3A+https%3A%2F%2Fdeveloper.mozilla.org%2Fen-US%2Fdocs%2FWeb%2FJavaScript%2FReference%2FGlobal_Objects%2FArray%0A*+GitHub+URL%3A+https%3A%2F%2Fgithub.com%2Fmdn%2Fcontent%2Fblob%2Fmain%2Ffiles%2Fen-us%2Fweb%2Fjavascript%2Freference%2Fglobal_objects%2Farray%2Findex.md%0A*+Last+commit%3A+https%3A%2F%2Fgithub.com%2Fmdn%2Fcontent%2Fcommit%2Fdd88a6eb2176fa31f5b744d8964efecf3f1f425b%0A*+Document+last+modified%3A+2026-02-24T22%3A43%3A48.000Z%0A%0A%3C%2Fdetails%3E "This will take you to GitHub to file a new issue."
[154]: /
[155]: https://github.com/mdn/
[156]: https://bsky.app/profile/developer.mozilla.org
[157]: https://x.com/mozdevnet
[158]: https://mastodon.social/@mdn
[159]: /en-US/blog/rss.xml
[160]: /en-US/about
[161]: /en-US/blog/
[162]: https://www.mozilla.org/en-US/careers/listings/
[163]: /en-US/advertising
[164]: /en-US/plus
[165]: https://support.mozilla.org/products/mdn-plus
[166]: /en-US/community
[167]: /en-US/docs/MDN/Community
[168]: /en-US/docs/MDN/Writing_guidelines
[169]: /discord
[170]: https://github.com/mdn
[171]: /en-US/docs/Learn_web_development
[172]: /en-US/docs/MDN/Guides
[173]: /en-US/docs/MDN/Tutorials
[174]: /en-US/docs/Glossary
[175]: https://hacks.mozilla.org/
[176]: https://www.mozilla.org/
[177]: https://www.mozilla.org/privacy/websites/
[178]: https://www.mozilla.org/en-US/privacy/websites/data-preferences/
[179]: https://www.mozilla.org/about/legal/terms/mozilla
[180]: https://www.mozilla.org/about/governance/policies/participation/
[181]: https://foundation.mozilla.org/
[182]: /docs/MDN/Writing_guidelines/Attrib_copyright_license
