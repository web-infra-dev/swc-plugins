import { createContainer as useContainer } from "@modern123-js/runtime";
useContainer({
    loader: function() {
        var innerLoader = foo2;
        innerLoader.id = "d7919ac9a4465387c91457f8c62ccb7d_0";
        return innerLoader;
    }(),
    staticLoader: function() {
        var innerLoader = bar2;
        innerLoader.id = "d7919ac9a4465387c91457f8c62ccb7d_1";
        return innerLoader;
    }()
});
