## react-constant-elements

This plugin can hoist static v-dom nodes to the right scope.

Turns this:

```jsx
const App = () => {

  return <div>Hello World</div>
}

const createContent = (msg) => {

  return () => {
    return <div onClick={() => {console.log(msg)}}>
      <p>lorem ipsum</p>
    </div>
  }
}
```

Into this:

```jsx
let _div;
const App = () => {

  return _div || (_div = <div>Hello World</div>)
}

const createContent = (msg) => {
  let _p;
  return () => {
    return <div onClick={() => {console.log(msg)}}>
      {_p || (_p = <p>lorem ipsum</p>)}
    </div>
  }
}
```

This could save memories during react state changes. Static v-dom nodes are only created once.

## Options

### immutable_globals

- type: `Vec<String>`

You can specify immutable_globals to let plugin decide which element can be hoisted if its children are immutable. For example consider this: ```<App />```, App is global variable, so we won't hoist it, if you are sure App won't change, you can add `App` to immutable_globals.

### allow_mutable_props_on_tags

- type: `Vec<String>`

If a JSX element can be hoisted, it won't have any expression expect `Ident` in JSX. For example ```<Comp style={ { color: 'red' } }>```, the JSX contains object literal, it is not allowed to hoist, but if you are sure this is safe, you can add `Comp` to allow_mutable_props_on_tags.
