window.BENCHMARK_DATA = {
  "lastUpdate": 1668693239490,
  "repoUrl": "https://github.com/modern-js-dev/swc-plugins",
  "entries": {
    "Benchmark": [
      {
        "commit": {
          "author": {
            "email": "1114550440@qq.com",
            "name": "fengyu",
            "username": "JSerFeng"
          },
          "committer": {
            "email": "1114550440@qq.com",
            "name": "Fy",
            "username": "JSerFeng"
          },
          "distinct": true,
          "id": "ee3aa41e1bcaa5608a051dda502aa05eb9f7dc01",
          "message": "bench: compare with @swc/core minify",
          "timestamp": "2022-11-16T12:30:52+08:00",
          "tree_id": "fc99736b4fae1fd95f4672790c17934f03bcb828",
          "url": "https://github.com/modern-js-dev/swc-plugins/commit/ee3aa41e1bcaa5608a051dda502aa05eb9f7dc01"
        },
        "date": 1668574259693,
        "tool": "cargo",
        "benches": [
          {
            "name": "minify_large_bundle_no_sourcemap",
            "value": 929664917,
            "range": "± 37510246",
            "unit": "ns/iter"
          },
          {
            "name": "minify_large_bundle_with_sourcemap",
            "value": 1026542668,
            "range": "± 32559937",
            "unit": "ns/iter"
          },
          {
            "name": "swc_core_minify",
            "value": 1034241743,
            "range": "± 27614252",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "1114550440@qq.com",
            "name": "fengyu",
            "username": "JSerFeng"
          },
          "committer": {
            "email": "1114550440@qq.com",
            "name": "Fy",
            "username": "JSerFeng"
          },
          "distinct": true,
          "id": "b436c72fe630ac399697a97e59d73b00f6a16c6b",
          "message": "chore(release): publish",
          "timestamp": "2022-11-16T15:49:09+08:00",
          "tree_id": "b86ce9101d27ca421ba81231eb9e2df7a4e1e3ef",
          "url": "https://github.com/modern-js-dev/swc-plugins/commit/b436c72fe630ac399697a97e59d73b00f6a16c6b"
        },
        "date": 1668586256699,
        "tool": "cargo",
        "benches": [
          {
            "name": "minify_large_bundle_no_sourcemap",
            "value": 1051841484,
            "range": "± 90116300",
            "unit": "ns/iter"
          },
          {
            "name": "minify_large_bundle_with_sourcemap",
            "value": 1158474828,
            "range": "± 79496406",
            "unit": "ns/iter"
          },
          {
            "name": "swc_core_minify",
            "value": 1045489434,
            "range": "± 145645879",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "modern-js-dev",
            "username": "modern-js-dev"
          },
          "committer": {
            "name": "modern-js-dev",
            "username": "modern-js-dev"
          },
          "id": "5e01872939912813ede5b135ffb12a8436e6b763",
          "message": "Chore/release",
          "timestamp": "2022-11-16T04:30:58Z",
          "url": "https://github.com/modern-js-dev/swc-plugins/pull/33/commits/5e01872939912813ede5b135ffb12a8436e6b763"
        },
        "date": 1668693237959,
        "tool": "cargo",
        "benches": [
          {
            "name": "minify_large_bundle_no_sourcemap",
            "value": 1187174480,
            "range": "± 84330049",
            "unit": "ns/iter"
          },
          {
            "name": "minify_large_bundle_with_sourcemap",
            "value": 1311912793,
            "range": "± 80136070",
            "unit": "ns/iter"
          },
          {
            "name": "swc_core_minify",
            "value": 1207360171,
            "range": "± 140343855",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}