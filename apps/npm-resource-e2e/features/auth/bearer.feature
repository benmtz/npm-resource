Feature: Auth

  Background:
    Given registry is up
    And registry is ready for the following publications :
      | package | version |
      | my_package | version |
    And registry has the following packages :
      | package      | version |
      | my_package   | 0.0.0   |
    And registry has the following users :
      | username | password | token |
      | user     | p@ssw0rd | t0ken |

  Scenario Outline:
    Given we execute the in
    And into a temporary directory
    And resource source is :
      """
      registry: http://localhost:1080
      package_name: <package_name>
      auth_type: bearer
      token: <token>
      """
    And parameters are empty
    And checked version is <version>
    When we execute the command
    Then <package_name> manifest has been called with token <token>
    And <package_name> has been downloaded in version <version> with token <token>
    And the file package.json is in the temporary directory
    Examples:
      | token | package_name | version |
      | t0ken | my_package   | 0.0.0   |

  Scenario Outline:
    Given we execute the out
    And we provide the arg : apps/npm-resource-e2e/assets
    And resource source is :
      """
      registry: http://localhost:1080
      package_name: <package_name>
      auth_type: bearer
      token: <token>
      """
    And params is :
      """
      package: smallest_package
      package_name: <package_name>
      version: <version>
      """
    And checked version is <version>
    When we execute the command
    Then <package_name> has been uploaded with token <token>
    Examples:
      | token | package_name | version |
      | t0ken | my_package   | 0.0.0   |
