# 자동차 경주 게임

## 기능 구현

- [x] 사용자는 자동차들의 이름을 입력한다.
  - [x] 자동차 이름은 쉼표(,)를 기준으로 구분한다.
  - [x] 자동차의 이름으로 공백을 입력하면 예외가 발생한다.
  - [x] 자동차의 이름이 5자를 초과하면 예외가 발생한다.
  - [x] 입력된 자동차명이 없는 경우 예외가 발생한다.

- [x] 사용자는 해당 게임이 몇 개의 라운드로 구성되었는지를 입력한다.
  - [x] 숫자를 입력하지 않으면 예외가 발생한다.
  - [x] 양의 정수를 입력하지 않으면 예외가 발생한다.

- [x] 매 라운드마다 각 자동차는 전진 혹은 정지한다.
  - [x] 각 자동차는 0~9 사이의 무작위 값을 구한 후 무작위 값이 4 이상일 경우에 한 칸 전진한다.
  - [x] 매 라운드마다 자동차의 총 이동거리를 해당 자동차의 이름과 함께 출력한다.

- [ ] 모든 라운드가 종료되었을 때 우승자를 출력한다.
  - 우승자가 여러 명일 경우 쉼표(,)를 이용하여 구분한다.

- [ ] 사용자가 잘못된 값을 입력할 경우 예외를 발생시키고, `[ERROR]`로 시작하는 에러 메시지를 출력하고 해당 부분부터 입력을 다시 받는다.
