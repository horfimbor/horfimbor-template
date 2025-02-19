Feature: Template example

  Scenario: i can add a number to a new Template
   Given a template
    When i try to add 50
    Then it got a value of 1387

  Scenario: i can add too much
    Given a template
    When i try to add 50000
    Then it got an error