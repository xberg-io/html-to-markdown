---
canonical: https://react.dev/learn
meta-algolia-search-order: 3
meta-fb:app_id: 623268441017527
meta-google-site-verification: sIlAGs48RulR4DdP95YSWNKZIEtCqQmRjzn-Zq-CcD0
meta-msapplication-TileColor: #2b5797
meta-og:description: The library for web and native user interfaces
meta-og:image: https://react.dev/images/og-learn.png
meta-og:title: Quick Start – React
meta-og:type: website
meta-og:url: https://react.dev/learn
meta-theme-color: #23272f
meta-twitter:card: summary_large_image
meta-twitter:creator: @reactjs
meta-twitter:description: The library for web and native user interfaces
meta-twitter:image: https://react.dev/images/og-learn.png
meta-twitter:site: @reactjs
meta-twitter:title: Quick Start – React
meta-viewport: width=device-width, initial-scale=1
title: Quick Start – React
---


[Learn React][1]

Copy pageCopy

## Quick Start[#undefined][2]

Welcome to the React documentation! This page will give you an introduction to 80% of the React concepts that you will use on a daily basis.

### You will learn

- How to create and nest components
- How to add markup and styles
- How to display data
- How to render conditions and lists
- How to respond to events and update the screen
- How to share data between components

### Creating and nesting components [#components][3]

React apps are made out of *components*. A component is a piece of the UI (user interface) that has its own logic and appearance. A component can be as small as a button, or as large as an entire page.

React components are JavaScript functions that return markup:

```text
function MyButton() {

  return (

    <button>I'm a button</button>

  );

}
```

Now that you’ve declared `MyButton`, you can nest it into another component:

```text
export default function MyApp() {

  return (

    <div>

      <h1>Welcome to my app</h1>

      <MyButton />

    </div>

  );

}
```

Notice that `<MyButton />` starts with a capital letter. That’s how you know it’s a React component. React component names must always start with a capital letter, while HTML tags must be lowercase.

Have a look at the result:

App.js

App.js

Reload

Clear

[Fork][4]

```text
function MyButton() {
  return (
    <button>
      I'm a button
    </button>
  );
}

export default function MyApp() {
  return (
    <div>
      <h1>Welcome to my app</h1>
      <MyButton />
    </div>
  );
}
```

Show more

The `export default` keywords specify the main component in the file. If you’re not familiar with some piece of JavaScript syntax, [MDN][5] and [javascript.info][6] have great references.

### Writing markup with JSX [#writing-markup-with-jsx][7]

The markup syntax you’ve seen above is called *JSX*. It is optional, but most React projects use JSX for its convenience. All of the [tools we recommend for local development][8] support JSX out of the box.

JSX is stricter than HTML. You have to close tags like `<br />`. Your component also can’t return multiple JSX tags. You have to wrap them into a shared parent, like a `<div>...</div>` or an empty `<>...</>` wrapper:

```text
function AboutPage() {

  return (

    <>

      <h1>About</h1>

      <p>Hello there.<br />How do you do?</p>

    </>

  );

}
```

If you have a lot of HTML to port to JSX, you can use an [online converter.][9]

### Adding styles [#adding-styles][10]

In React, you specify a CSS class with `className`. It works the same way as the HTML [`class`][11] attribute:

```text
<img className="avatar" />
```

Then you write the CSS rules for it in a separate CSS file:

```text
/* In your CSS */

.avatar {

  border-radius: 50%;

}
```

React does not prescribe how you add CSS files. In the simplest case, you’ll add a [`<link>`][12] tag to your HTML. If you use a build tool or a framework, consult its documentation to learn how to add a CSS file to your project.

### Displaying data [#displaying-data][13]

JSX lets you put markup into JavaScript. Curly braces let you “escape back” into JavaScript so that you can embed some variable from your code and display it to the user. For example, this will display `user.name`:

```text
return (

  <h1>

    {user.name}

  </h1>

);
```

You can also “escape into JavaScript” from JSX attributes, but you have to use curly braces *instead of* quotes. For example, `className="avatar"` passes the `"avatar"` string as the CSS class, but `src={user.imageUrl}` reads the JavaScript `user.imageUrl` variable value, and then passes that value as the `src` attribute:

```text
return (

  <img

    className="avatar"

    src={user.imageUrl}

  />

);
```

You can put more complex expressions inside the JSX curly braces too, for example, [string concatenation][14]:

App.js

App.js

Reload

Clear

[Fork][4]

```text
const user = {
  name: 'Hedy Lamarr',
  imageUrl: 'https://i.imgur.com/yXOvdOSs.jpg',
  imageSize: 90,
};

export default function Profile() {
  return (
    <>
      <h1>{user.name}</h1>
      <img
        className="avatar"
        src={user.imageUrl}
        alt={'Photo of ' + user.name}
        style={{
          width: user.imageSize,
          height: user.imageSize
        }}
      />
    </>
  );
}
```

Show more

In the above example, `style={{}}` is not a special syntax, but a regular `{}` object inside the `style={ }` JSX curly braces. You can use the `style` attribute when your styles depend on JavaScript variables.

### Conditional rendering [#conditional-rendering][15]

In React, there is no special syntax for writing conditions. Instead, you’ll use the same techniques as you use when writing regular JavaScript code. For example, you can use an [`if`][16] statement to conditionally include JSX:

```text
let content;

if (isLoggedIn) {

  content = <AdminPanel />;

} else {

  content = <LoginForm />;

}

return (

  <div>

    {content}

  </div>

);
```

If you prefer more compact code, you can use the [conditional `?` operator.][17] Unlike `if`, it works inside JSX:

```text
<div>

  {isLoggedIn ? (

    <AdminPanel />

  ) : (

    <LoginForm />

  )}

</div>
```

When you don’t need the `else` branch, you can also use a shorter [logical `&&` syntax][18]:

```text
<div>

  {isLoggedIn && <AdminPanel />}

</div>
```

All of these approaches also work for conditionally specifying attributes. If you’re unfamiliar with some of this JavaScript syntax, you can start by always using `if...else`.

### Rendering lists [#rendering-lists][19]

You will rely on JavaScript features like [`for` loop][20] and the [array `map()` function][21] to render lists of components.

For example, let’s say you have an array of products:

```text
const products = [

  { title: 'Cabbage', id: 1 },

  { title: 'Garlic', id: 2 },

  { title: 'Apple', id: 3 },

];
```

Inside your component, use the `map()` function to transform an array of products into an array of `<li>` items:

```text
const listItems = products.map(product =>

  <li key={product.id}>

    {product.title}

  </li>

);


return (

  <ul>{listItems}</ul>

);
```

Notice how `<li>` has a `key` attribute. For each item in a list, you should pass a string or a number that uniquely identifies that item among its siblings. Usually, a key should be coming from your data, such as a database ID. React uses your keys to know what happened if you later insert, delete, or reorder the items.

App.js

App.js

Reload

Clear

[Fork][4]

```text
const products = [
  { title: 'Cabbage', isFruit: false, id: 1 },
  { title: 'Garlic', isFruit: false, id: 2 },
  { title: 'Apple', isFruit: true, id: 3 },
];

export default function ShoppingList() {
  const listItems = products.map(product =>
    <li
      key={product.id}
      style={{
        color: product.isFruit ? 'magenta' : 'darkgreen'
      }}
    >
      {product.title}
    </li>
  );

  return (
    <ul>{listItems}</ul>
  );
}
```

Show more

### Responding to events [#responding-to-events][22]

You can respond to events by declaring *event handler* functions inside your components:

```text
function MyButton() {

  function handleClick() {

    alert('You clicked me!');

  }


  return (

    <button onClick={handleClick}>

      Click me

    </button>

  );

}
```

Notice how `onClick={handleClick}` has no parentheses at the end! Do not *call* the event handler function: you only need to *pass it down*. React will call your event handler when the user clicks the button.

### Updating the screen [#updating-the-screen][23]

Often, you’ll want your component to “remember” some information and display it. For example, maybe you want to count the number of times a button is clicked. To do this, add *state* to your component.

First, import [`useState`][24] from React:

```text
import { useState } from 'react';
```

Now you can declare a *state variable* inside your component:

```text
function MyButton() {

  const [count, setCount] = useState(0);

  // ...
```

You’ll get two things from `useState`: the current state (`count`), and the function that lets you update it (`setCount`). You can give them any names, but the convention is to write `[something, setSomething]`.

The first time the button is displayed, `count` will be `0` because you passed `0` to `useState()`. When you want to change state, call `setCount()` and pass the new value to it. Clicking this button will increment the counter:

```text
function MyButton() {

  const [count, setCount] = useState(0);


  function handleClick() {

    setCount(count + 1);

  }


  return (

    <button onClick={handleClick}>

      Clicked {count} times

    </button>

  );

}
```

React will call your component function again. This time, `count` will be `1`. Then it will be `2`. And so on.

If you render the same component multiple times, each will get its own state. Click each button separately:

App.js

App.js

Reload

Clear

[Fork][4]

```text
import { useState } from 'react';

export default function MyApp() {
  return (
    <div>
      <h1>Counters that update separately</h1>
      <MyButton />
      <MyButton />
    </div>
  );
}

function MyButton() {
  const [count, setCount] = useState(0);

  function handleClick() {
    setCount(count + 1);
  }

  return (
    <button onClick={handleClick}>
      Clicked {count} times
    </button>
  );
}
```

Show more

Notice how each button “remembers” its own `count` state and doesn’t affect other buttons.

### Using Hooks [#using-hooks][25]

Functions starting with `use` are called *Hooks*. `useState` is a built-in Hook provided by React. You can find other built-in Hooks in the [API reference.][26] You can also write your own Hooks by combining the existing ones.

Hooks are more restrictive than other functions. You can only call Hooks *at the top* of your components (or other Hooks). If you want to use `useState` in a condition or a loop, extract a new component and put it there.

### Sharing data between components [#sharing-data-between-components][27]

In the previous example, each `MyButton` had its own independent `count`, and when each button was clicked, only the `count` for the button clicked changed:

*Initially, each `MyButton`’s `count` state is `0`*

*The first `MyButton` updates its `count` to `1`*

However, often you’ll need components to *share data and always update together*.

To make both `MyButton` components display the same `count` and update together, you need to move the state from the individual buttons “upwards” to the closest component containing all of them.

In this example, it is `MyApp`:

*Initially, `MyApp`’s `count` state is `0` and is passed down to both children*

*On click, `MyApp` updates its `count` state to `1` and passes it down to both children*

Now when you click either button, the `count` in `MyApp` will change, which will change both of the counts in `MyButton`. Here’s how you can express this in code.

First, *move the state up* from `MyButton` into `MyApp`:

```text
export default function MyApp() {

  const [count, setCount] = useState(0);


  function handleClick() {

    setCount(count + 1);

  }


  return (

    <div>

      <h1>Counters that update separately</h1>

      <MyButton />

      <MyButton />

    </div>

  );

}


function MyButton() {

  // ... we're moving code from here ...

}
```

Then, *pass the state down* from `MyApp` to each `MyButton`, together with the shared click handler. You can pass information to `MyButton` using the JSX curly braces, just like you previously did with built-in tags like `<img>`:

```text
export default function MyApp() {

  const [count, setCount] = useState(0);


  function handleClick() {

    setCount(count + 1);

  }


  return (

    <div>

      <h1>Counters that update together</h1>

      <MyButton count={count} onClick={handleClick} />

      <MyButton count={count} onClick={handleClick} />

    </div>

  );

}
```

The information you pass down like this is called *props*. Now the `MyApp` component contains the `count` state and the `handleClick` event handler, and *passes both of them down as props* to each of the buttons.

Finally, change `MyButton` to *read* the props you have passed from its parent component:

```text
function MyButton({ count, onClick }) {

  return (

    <button onClick={onClick}>

      Clicked {count} times

    </button>

  );

}
```

When you click the button, the `onClick` handler fires. Each button’s `onClick` prop was set to the `handleClick` function inside `MyApp`, so the code inside of it runs. That code calls `setCount(count + 1)`, incrementing the `count` state variable. The new `count` value is passed as a prop to each button, so they all show the new value. This is called “lifting state up”. By moving state up, you’ve shared it between components.

App.js

App.js

Reload

Clear

[Fork][4]

```text
import { useState } from 'react';

export default function MyApp() {
  const [count, setCount] = useState(0);

  function handleClick() {
    setCount(count + 1);
  }

  return (
    <div>
      <h1>Counters that update together</h1>
      <MyButton count={count} onClick={handleClick} />
      <MyButton count={count} onClick={handleClick} />
    </div>
  );
}

function MyButton({ count, onClick }) {
  return (
    <button onClick={onClick}>
      Clicked {count} times
    </button>
  );
}
```

Show more

### Next Steps [#next-steps][36]

By now, you know the basics of how to write React code!

Check out the [Tutorial][37] to put them into practice and build your first mini-app with React.

[NextTutorial: Tic-Tac-Toe][37]

---

[https://opensource.fb.com/][38]

Copyright © Meta Platforms, Inc

no uwu plz

uwu?

Logo by[@sawaratsuki1004][39]

[Learn React][1]

[Quick Start][1]

[Installation][8]

[Describing the UI][40]

[Adding Interactivity][41]

[Managing State][42]

[Escape Hatches][43]

[API Reference][26]

[React APIs][26]

[React DOM APIs][44]

[Community][45]

[Code of Conduct][46]

[Meet the Team][47]

[Docs Contributors][48]

[Acknowledgements][49]

More

[Blog][50]

[React Native][51]

[Privacy][52]

[Terms][53]

[https://www.facebook.com/react][54][https://twitter.com/reactjs][55][https://bsky.app/profile/react.dev][56][https://github.com/facebook/react][57]



[1]: /learn
[2]: #undefined "Link for this heading"
[3]: #components "Link for Creating and nesting components "
[4]: https://codesandbox.io/api/v1/sandboxes/define?undefined&environment=create-react-app "Open in CodeSandbox"
[5]: https://developer.mozilla.org/en-US/docs/web/javascript/reference/statements/export
[6]: https://javascript.info/import-export
[7]: #writing-markup-with-jsx "Link for Writing markup with JSX "
[8]: /learn/installation
[9]: https://transform.tools/html-to-jsx
[10]: #adding-styles "Link for Adding styles "
[11]: https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/class
[12]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/link
[13]: #displaying-data "Link for Displaying data "
[14]: https://javascript.info/operators#string-concatenation-with-binary
[15]: #conditional-rendering "Link for Conditional rendering "
[16]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/if...else
[17]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Conditional_Operator
[18]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Logical_AND#short-circuit_evaluation
[19]: #rendering-lists "Link for Rendering lists "
[20]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/for
[21]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/map
[22]: #responding-to-events "Link for Responding to events "
[23]: #updating-the-screen "Link for Updating the screen "
[24]: /reference/react/useState
[25]: #using-hooks "Link for Using Hooks "
[26]: /reference/react
[27]: #sharing-data-between-components "Link for Sharing data between components "
[28]: /_next/image?url=%2Fimages%2Fdocs%2Fdiagrams%2Fsharing_data_child.dark.png&amp;w=828&amp;q=75
[29]: /_next/image?url=%2Fimages%2Fdocs%2Fdiagrams%2Fsharing_data_child.png&amp;w=828&amp;q=75
[30]: /_next/image?url=%2Fimages%2Fdocs%2Fdiagrams%2Fsharing_data_child_clicked.dark.png&amp;w=828&amp;q=75
[31]: /_next/image?url=%2Fimages%2Fdocs%2Fdiagrams%2Fsharing_data_child_clicked.png&amp;w=828&amp;q=75
[32]: /_next/image?url=%2Fimages%2Fdocs%2Fdiagrams%2Fsharing_data_parent.dark.png&amp;w=828&amp;q=75
[33]: /_next/image?url=%2Fimages%2Fdocs%2Fdiagrams%2Fsharing_data_parent.png&amp;w=828&amp;q=75
[34]: /_next/image?url=%2Fimages%2Fdocs%2Fdiagrams%2Fsharing_data_parent_clicked.dark.png&amp;w=828&amp;q=75
[35]: /_next/image?url=%2Fimages%2Fdocs%2Fdiagrams%2Fsharing_data_parent_clicked.png&amp;w=828&amp;q=75
[36]: #next-steps "Link for Next Steps "
[37]: /learn/tutorial-tic-tac-toe
[38]: https://opensource.fb.com/
[39]: https://twitter.com/sawaratsuki1004
[40]: /learn/describing-the-ui
[41]: /learn/adding-interactivity
[42]: /learn/managing-state
[43]: /learn/escape-hatches
[44]: /reference/react-dom
[45]: /community
[46]: https://github.com/facebook/react/blob/main/CODE_OF_CONDUCT.md
[47]: /community/team
[48]: /community/docs-contributors
[49]: /community/acknowledgements
[50]: /blog
[51]: https://reactnative.dev/
[52]: https://opensource.facebook.com/legal/privacy
[53]: https://opensource.fb.com/legal/terms/
[54]: https://www.facebook.com/react
[55]: https://twitter.com/reactjs
[56]: https://bsky.app/profile/react.dev
[57]: https://github.com/facebook/react
