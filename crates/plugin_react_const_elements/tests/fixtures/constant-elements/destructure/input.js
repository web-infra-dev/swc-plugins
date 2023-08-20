class AnchorLink extends Component {
  render() {
    const { isExternal, children } = this.props;
    if (isExternal) {
      return (<a>immutable</a>);
    }

    return (<Link>{children}</Link>);
  }
}
