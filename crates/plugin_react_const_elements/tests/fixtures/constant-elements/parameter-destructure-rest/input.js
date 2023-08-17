function render({ text, className, id, ...props }) {
  // intentionally ignoring props
  return () => (<div text={text} className={className} id={id} />);
}
