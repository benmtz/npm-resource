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
    Given we execute the out
    And we provide the arg : this_directory_does_not_exist
    And resource source is :
      """
      registry: http://localhost:1080
      package_name: <package_name>
      """
    And params is :
      """
      package: smallest_package
      """
    And checked version is <version>
    When we execute the command
    Then The command exited with a code 1
    And The command threw a message containing "No such file or directory"
    Examples:
      | token | package_name | version |
      | t0ken | my_package   | 0.0.0   |
