# 데코레이터 패턴

- 객체에 추가적인 요건을 동적으로 첨가한다. 데코레이터는 서브클래스를 만드는 것을 통해 기능을 유연하게 확장할 수 있는 방법을 제공한다.
- 데코레이터의 수퍼클래스는 자신이 장식하고 있는 객체의 수퍼클래스와 같다
- 한 객체를 여러 개의 데코레이터로 감쌀 수 있습니다
- 데코레이터는 자신이 감싸고 있는 객체와 같은 수퍼클래스를 가지고 있기 때문에 원래 객체가 들어갈 자리에 데코레이터 객체를 집어넣어도 된다
- 데코레이터는 자신이 장식하고 있는 객체에게 어떤 행동을 위임하는 것 외에 원하는 추가적인 작업을 수행할 수 있다
- 객체는 언제든지 감쌀 수 있기 때문에 실행 중에 필요한 데코레이터를 마음대로 적용할 수 있다
- 일반적으로 빌더나 팩토리 같은 다른 패턴을 써서 만들어서 실수로 잘못 데코레이팅할 확률을 줄인다
- 구성과 위임을 통해 실행 중에 새로운 행동을 추가할 수 있다
- 데코레이터 패턴을 사용하면 자잘한 객체들이 매우 많이 추가될 수 있고, 데코레이터를 너무 많이 사용하면 코드가 필요 이상으로 복잡해질 수도 있다
- 자바IO에서 데코레이터 패턴을 적용했다: BufferedInputStream, LineNumberInputStream 모두 FileInputStream을 확장함

## 디자인 원칙
- OCP