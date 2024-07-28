# Argo Workflows OpenAPI

This crate provides Rust definitions for accessing the [Argo Workflows REST API][3].
The models and API were first auto-generated from the
[Argo Workflows OpenAPI spec][0], and then hand crafted to build a library that
is easy to use.

# License

```
argo-workflows-openapi

https://github.com/cubenix/argo-workflows-openapi

Copyright 2023 Gaurav Gahlot

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```

The OpenAPI spec that these bindings are generated from is sourced from the
[Argo Workflows repository][2] which also uses the Apache-2.0 license.

[0]: https://github.com/argoproj/argo-workflows/blob/main/api/openapi-spec/swagger.json
[1]: https://argoproj.github.io/argo-workflows/client-libraries/
[2]: https://github.com/argoproj/argo-workflows/
[3]: https://argoproj.github.io/argo-workflows/swagger/
