Feature: Template example

  Scenario: i can empty a new Template
   Given a template
    When i try to reset it
    Then it got a value of 0

  Scenario: i cant empty twice Template
    Given a template
    When i try to reset it
    When i try to reset it
    Then it got an error
