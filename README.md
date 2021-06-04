# npm-resource

This is a concourse npm-resource written in rust, if you intend on using this resource there is limitation to keep in mind :
it wont run npm scripts, since this resource publish directly to registries api through http(s)

## Usage

include the resource type into your pipeline descriptor

```yaml
resource_types:
  - name: npm-resource
    type: registry-image
    source:
      repository: benmtz/npm-resource
      tag: latest
```

## Source Configuration

| parameter    | needed                       | description                                                     |
| ------------ | ---------------------------- | --------------------------------------------------------------- |
| package_name | Required                     | package_name (with scope), it will be use in check and in steps |
| registry     | Optional                     | url to registry                                                 |
| token        | Required                     | token to use against registry                                   |
| tag          | Optional (default to latest) | tag to look for, (latest, semver or any string)                 |

## Behaviour

### check: Check for new version of specified npm package.

Checks for new versions of a specific **tag** of **package_name** into the **registry**

```yaml
source:
  # Check for latest tags, (we could omit the latest tag since it's the default)
  registry: ((npm.registry))
  package_name: ((app.lib_name))
  token: ((npm.token))
  tag: latest

source:
  # Check for next tags
  registry: ((npm.registry))
  package_name: ((app.lib_name))
  token: ((npm.token))
  tag: next
```

### in: Fetch and extract the package tarball

There is no specific parameter for this behaviour

### out: Publish tagged npm package to registry

- to tag an existing version : provide a version and no package
- to publish a new version : provide a package

#### Parameters

| Parameter    | Needed   | Description                                                                                                                     |
| ------------ | -------- | ------------------------------------------------------------------------------------------------------------------------------- |
| package      | Required | Path to the package to publish, either a tar.gz produced by npm pack or a folder with a package.json                            |
| version      | Optional | Publish version, if version is a valid semver then it will be used as provide, else we read the version file to extract version |
| package_name | Optional | Override package name                                                                                                           |

## Pipeline example

- [Check exemple](examples/pipeline/check-examples.yaml)
- [Simple pipeline pushing a project to next tag then promoting it to latest](examples/pipeline/simple-pipeline-next-latest.yaml)

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  http://opensource.org/licenses/MIT)

at your option.
