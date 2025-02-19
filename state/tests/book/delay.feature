Feature: Template example

  Scenario: i can add a number to a new Template after a delay
   Given a template
    When i try to delay 50 by 2 secs
    Then it got a value of 1337

  Scenario: i cannot delay by 0 secs
   Given a template
    When i try to delay 50 by 0 secs
    Then it got an error

  Scenario: i cannot delay by more than 10 secs
   Given a template
    When i try to delay 50 by 11 secs
    Then it got an error


  Scenario: i cannot finalized a delay before the end
    Given a template
    When i try to delay 100 by 9 secs
    When i wait 5 seconds
    When i try to finalize the delay 0
    Then it got an error

  Scenario: i can finalized a delay after the end
    Given a template
    When i try to delay 100 by 9 secs
    Then remaining delay is 1
    When i wait 10 seconds
    When i try to finalize the delay 0
    Then it got a value of 1437
    Then remaining delay is 0


  Scenario: i can add multiple delay
    Given a template
    When i try to delay 42 by 8 secs
    When i try to delay 58 by 3 secs
    Then remaining delay is 2
    When i wait 5 seconds
    When i try to finalize the delay 1
    Then it got a value of 1395
    Then remaining delay is 1
