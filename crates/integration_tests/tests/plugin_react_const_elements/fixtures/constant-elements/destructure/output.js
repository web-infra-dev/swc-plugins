let _a;
class AnchorLink extends Component {
  render() {
    const { isExternal, children } = this.props;
    if (isExternal) {
      return _a || (_a = <a>immutable</a>);
    }
    return <Link>{children}</Link>;
  }
}
