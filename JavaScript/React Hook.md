# React Hook (16.8~)

## 왜?
1. Wrapper Hell
    - 컴포넌트가 너무 복잡한 계층적 구조로 연결됨 => 컴포넌트 관리의 어려움
    - Hook: 상태 관련 로직을 추상화, 계층 변화 없이 상태 관련 로직을 재사용
2. 너무 큰 컴포넌트
    - 지나치게 커지고 복잡해지고, UI 상에서 제어할 것이 너무 많아짐
    - Hook: 로직에 기반을 둔 작은 함수로 컴포넌트를 나눌 수 있음
3. class, this, bind
    - 클래스 컴포넌트로 인해 this, bind를 사용하게 되어 혼란을 가져옴
    - Hook: 그럴 필요가 없음 (함수형 컴포넌트로 작성)

## 무엇인가?
- 상태와 라이프사이클을 연동(hook into)할 수 있게 하는 함수.
- 리액트는 useState과 같은 내장 hook을 제공함.
- state 그 자체가 아니라, 상태 관련 로직을 재사용하는 방법 (-> 커스텀 훅)
    실제로 각각의 Hook 호출은 완전히 독립된 state를 가집니다. 그래서 심지어는 한 컴포넌트 안에서 같은 custom Hook을 두 번 쓸 수도 있습니다.

## 장점
1. 가독성
```javascript
 <p>You clicked {this.state.count} times</p> //class
 <p>You clicked {count} times</p> //function

 <button onClick={() => this.setState({ count: this.state.count + 1 })}>
    Click me
</button> //class

<button onClick={() => setCount(count + 1)}>
    Click me
</button> //function
```
2. 손쉬운 상태 관리
    - 상태를 초기화하는 로직이 별도로 필요하지 않고 상단에 선언 가능
    - redux, mobx 같은 라이브러리 없이도 손쉬운 상태 관리 제공

## 사용 규칙 (lint로 확인 가능)
1. 최상위 레벨에서만 호출하기 (반복문, 조건문, 중첩된 함수 X)
2. 리액트 컴포넌트 안에서만 사용 가능
- 규칙을 어길까 걱정된다면... lint 에 eslint-plugin-react-hooks 추가

## 기본 Hook
### useState (상태 관리)
- 상태 변수를 관리할 수 있게 함.
- 현재의 state 값과 이 값을 업데이트하는 함수를 쌍으로 제공 (참고: 구조 분해 할당 (destruncturing))
```javascript
const [count, setCount] = useState(0);
// const [state변수, 이 변수를 갱신할 수 있는 함수] = useState(초기값);
```
- class의 this.setState와 거의 유사하지만, 이전 state와 새로운 state를 병합하는 것이 아니라 대체하는 것이다.
    * 배열 스테이트 값을 다른 변수에 할당하고 값을 조작한 다음에 상태 업데이트를 한다 -> 제대로 업데이트 되지 않음.

### useEffect (라이프사이클 관리)
- 함수 컴포넌트 내에서 side effects를 수행할 수 있게 함.
- side effect ? 데이터 가져오기, 구독(subscription) 설정하기, 수동으로 리액트 컴포넌트의 DOM을 수정
```javascript
useEffect(effectFunction);
```
- DOM 업데이트 완료 -> React가 effect 함수 호출
- 첫번째 렌더링과 이후 모든 업데이트(렌더)에서 실행. 모든 렌더링에서 effect는 교체되어 이전 effect와 다름.
- effect는 clean up 함수를 리턴값으로 가질 수 있다.
    * clean up이 필요하지 않은 경우 = 실행 이후 신경 쓸 것이 없음 (네트워크 리퀘스트, DOM 수동 조작, 로깅 등)
    * clean up이 필요한 경우 = 실행 이후에도 신경 쓸 것이 있음 (외부 데이터에 구독(subscription)을 설정해야 하는 경우 등. 메모리 릭이 나지 않게.)
    * 컴포넌트 마운트가 해제될 때 clean up을 실행하지만, 이펙트는 모든 렌더링에서 실행되므로 다음 이펙트 실행 전에 이전 이펙트를 정리해야 함.
```javascript
// clean up (X)
useEffect(() => { //이렇게 넘겨주는 함수를 "effect"라고 함
    document.title = `You clicked ${count} times`;
});

// clean up (O)
useEffect(() => {
    function handleStatusChange(status) {
      setIsOnline(status.isOnline);
    }
    ChatAPI.subscribeToFriendStatus(props.friend.id, handleStatusChange);
    return function cleanup() { //화살표 함수를 쓰거나 다른 이름을 사용해도 무방
      ChatAPI.unsubscribeFromFriendStatus(props.friend.id, handleStatusChange);
    };
  });
```
- 클래스형 라이프사이클에서는 컴포넌트의 라이프사이클에 따라 로직을 분배하는 느낌이었다면, Hook으로는 로직의 역할로 분배할 수 있다.
```javascript
// Before
class FriendStatusWithCounter extends React.Component {
  constructor(props) {
    super(props);
    this.state = { count: 0, isOnline: null };
    this.handleStatusChange = this.handleStatusChange.bind(this);
  }

  componentDidMount() { //마운트가 되면
    document.title = `You clicked ${this.state.count} times`;
    ChatAPI.subscribeToFriendStatus(
      this.props.friend.id,
      this.handleStatusChange
    );
  }

  componentDidUpdate() { //업데이트가 되면
    document.title = `You clicked ${this.state.count} times`;
  }

  componentWillUnmount() { //마운트가 해제되면
    ChatAPI.unsubscribeFromFriendStatus(
      this.props.friend.id,
      this.handleStatusChange
    );
  }

  handleStatusChange(status) {
    this.setState({
      isOnline: status.isOnline
    });
  }

// After
function FriendStatusWithCounter(props) {
  const [count, setCount] = useState(0);
  useEffect(() => {  //도큐먼트 타이틀 관리는 이 이펙트로 하자.
    document.title = `You clicked ${count} times`;
  });

  const [isOnline, setIsOnline] = useState(null);
  useEffect(() => { //구독 관리는 이 이펙트로 하자.
    function handleStatusChange(status) {
      setIsOnline(status.isOnline);
    }

    ChatAPI.subscribeToFriendStatus(props.friend.id, handleStatusChange);
    return () => {
      ChatAPI.unsubscribeFromFriendStatus(props.friend.id, handleStatusChange);
    };
  });
  // ...
}
```
- 이펙트가 매번 교체되는 것을 방지하고 싶다 = 이펙트를 건너뛰게 하고 싶다
```javascript
useEffect(() => {
  document.title = `You clicked ${count} times`;
}, [count]); // useEffect에서 두 번째 인수는 선택적이다. 여기에 배열을 넘기면 된다. count가 바뀔 때만 effect를 재실행함.
// 이 배열은 컴포넌트 범위 내에서 바뀌는 값들과 이펙트에 의해 사용되는 값을 모두 포함해야 한다. (위의 예제에서는 count)

// 위의 예제를 수정해 보자면,
useEffect(() => {
  function handleStatusChange(status) {
    setIsOnline(status.isOnline);
  }

  ChatAPI.subscribeToFriendStatus(props.friend.id, handleStatusChange);
  return () => {
    ChatAPI.unsubscribeFromFriendStatus(props.friend.id, handleStatusChange);
  };
}, [props.friend.id]); // props.friend.id가 바뀔 때만 재구독합니다.

/*
    이펙트 실행과 clean up을 (마운트와 마운트 해제 시에)딱 한 번씩만 실행하고 싶다면
    빈 배열([])을 두 번째 인수로 넘기면 된다.
    = 이 이펙트는 어떤 props, state에도 의존하지 않음
    = 즉, 재실행할 필요 없음
    이 경우 effect 안의 prop과 state는 초깃값을 유지하게 된다.
    setCount(count + 1); 를 빈 배열을 두 번째 인자로 가지는 이펙트 안에서 쓰면 오류가 나는 이유.
    setCount(c => c + 1); 처럼 count 스테이트 변수에 대한 의존성을 없애 주어야 한다.
*/

// 이상한 의존을 배열에 넣을까봐 걱정된다면... lint 에 exhaustive-deps 추가
```

### useContext
- 컴포넌트 간 값을 전달할 때, 복잡하게 여러 단계 거치지 않게 하기 위해 전역으로 값을 사용할 수 있게 하는 것. (Redux와 유사한 기능)
```javascript
// 선언부
const themes = {
  light: {
    foreground: "#000000",
    background: "#eeeeee"
  },
  dark: {
    foreground: "#ffffff",
    background: "#222222"
  }
};

const ThemeContext = React.createContext(themes.light);

function App() {
  return (
    <ThemeContext.Provider value={themes.dark}>
      <Toolbar />
    </ThemeContext.Provider>
  );
}

function Toolbar(props) {
  return (
    <div>
      <ThemedButton />
    </div>
  );
}

// 호출부
function ThemedButton() {
  const theme = useContext(ThemeContext);
  return (
    <button style={{ background: theme.background, color: theme.foreground }}>
      I am styled by theme context!
    </button>
  );
}
```
- 사용법
    * context 객체(React.createContext에서 반환된 값) 만들기
    * 이때 context의 현재 값은 <MyContext.Provider>의 value prop에 의해 결정됨
    * 위의 값이 갱신되면 렌더러가 트리거됨 (useContext를 호출한 컴포넌트는 context값이 변경되면 항상 리렌더 -> 줄이려면 메모이제이션)
- 주의
    * useContext로 전달한 인자는 context 객체 그 자체여야 함 (ex. useContext(MyContext) O, useContext(MyContext.Consumer) X)

## 추가 Hook
### useReducer
- useState와 비슷한 역할을 함. 컴포넌트의 상태 업데이트 로직을 컴포넌트에서 분리할 때 사용.
```javascript
const [state, dispatch] = useReducer(reducer, initialArg, init);
// dispatch: 액션을 발생시키는 함수. dispatch({ type: 'INCREMENT' }) 처럼 사용
//          만약 현재 state와 같은 값을 반환하는 경우 React는 자식을 리렌더링하거나 effect를 발생하지 않음.
// initialArg: 초기 스테이트. 처음부터 전달할 수도 있고, 세 번째 파라미터로 초기화를 지연할 수 있음
// init: 스테이트 초기화 함수. init(initialArg)에 초기 스테이트가 설정된다.
```
- reducer란?
    * 현재 상태(state)와 액션(action) 객체를 파라미터로 받아와서 새로운 상태를 반환해주는 함수를 말함.
    * 리턴값 = 컴포넌트가 지닐 새로운 상태
    * 액션 = 업데이트를 위한 정보
- 특징
    * 다수의 하윗값을 포함하는 복잡한 정적 로직을 만드는 경우
    * 다음 state가 이전 state에 의존적인 경우
    * 자세한 업데이트를 트리거 하는 컴포넌트의 성능을 최적화 => 콜백 대신 dispatch를 전달 할 수 있기 때문

### useRef
- 변경 가능한 값을 .current 속성에 담아 ref 객체로 리턴하는 함수
- ref는 특정 element의 현상을 발생시키는 역할을 함. (ex. input focus 이동, 동영상 재생)
- 왜? 가변값을 유지하는 데에 더 편리 (pure JS Object)
- 특징? 값이 변경되어도 이를 알리지 않는다 = .current가 바뀐다고 리렌더링 되지 않는다
1. 사용예 1: DOM을 직접 선택해야 함
```javascript
const UseRefExample = () => {
  const [string, setString] = useState('');
  const [stringList, setStringList] = useState([]);

  // 1. 여기에서 레퍼런스를 사용할 변수를 만들고
  const inputText = useRef();

  const change = useCallback((e) => {
    setString(e.target.value);
  }, []);

  const insert = useCallback(() => {
    const newList = stringList.slice();
    newList.push(string);
    setStringList(newList);

    // 3. 이렇게 포커싱할 수 있다.
    inputText.current.focus();
  }, [string, stringList]);

  const sum = useCallback((list) => {
    let stringSum = '';
    for(let value of list) {
      stringSum += value + ' ';
    }
    return stringSum;
  }, []);

  const result = useMemo(() => sum(stringList), [stringList, sum]);

  return (
    <div>
      <input type='text' ref={inputText} onChange={change}/> 
      { /**2.위와 같이 인풋 태그에 레퍼런스를 걸어주면**/}
      <button onClick={insert}>문자열 추가</button>
      {result}
    </div>
  )
}
```

2. 사용예 2: 함수형 컴포넌트에서 로컬 변수로 사용 (스테이트와 유사)
```javascript
const LocalVar = () => {
  const localVar = useRef(1); //클래스형 컴포넌트의 경우 localVar = 1; 정도로 쓰임 

  const increaseLocalVar = () => {
    localVar.current = localVar.current + 1;
  }

  return (
    <div>
      {localVar}
    </div>
  )
}
export default LocalVar;

// 스테이트와 비교
const [count, setCount] = useState(0) //console.log(count)의 결과가 0
const count = useRef(0) //console.log(count.current)의 결과가 0
```

### useImperativeHandle
- ref를 사용할 때 부모 컴포넌트에 노출되는 인스턴스 값을 사용자화(customizes)
    * forwardRef로 부모에서 자식에게 레퍼런스를 전달할 수 있음 -> 자식은 내가 부모로부터 레퍼런스를 받았는지 모름
    * useImperativeHandle로 부모에게 자식의 레퍼런스 실제 값을 보내는 대신 프록시 레퍼런스를 보낼 수 있다.
        (= 부모가 자식의 DOM을 직접 제어하지 않는다 = 독립성을 보장할 수 있다)
- ref를 외부에서 참조하는 경우가 적어서 많이 사용되지는 않음.
    * 사용 상황 예) 라이브러리 래핑 / 부모에게 자식의 메서드를 넘겨야 할 때
```javascript
// 사용 방법
useImperativeHandle(ref, createHandle, [deps])

// 사용예
function FancyInput(props, ref) {
  const inputRef = useRef();
  useImperativeHandle(ref, () => ({
    focus: () => {
      inputRef.current.focus();
    }
  }));
  return <input ref={inputRef} ... />;
}
FancyInput = forwardRef(FancyInput);

// 위 예제를 다른 컴포넌트에서 아래와 같이 사용하면 inputRef.current.focus() 호출 가능.
<FancyInput ref={inputRef} />
```

### useMemo
- 메모이제이션 된 값을 반환
- 메모이제이션 = 동일한 계산을 반복해야 할 때, 이전에 계산한 값을 메모리에 저장함으로써 동일한 계산의 반복 수행을 제거하여 프로그램 실행 속도를 빠르게 하는 기술
- 성능 최적화를 위해 계산된 값을 재사용할 수 있음
- 공식 문서에서는 성능 최적화로 사용할 수 있지만, 이 정의 그대로 항상 작동할 것이라고 여기지는 말라고 언급
```javascript
// 느린 컴포넌트
function MyComponent({ x, y }) {
  const z = computeExpensiveValue(x, y)
  return <div>{z}</div>
}

// 성능 최적화
function MyComponent({ x, y }) {
  const z = useMemo(() => computeExpensiveValue(x, y), [x, y])
  // useMemo에 전달된 함수는 렌더링 이후 실행되는 effect와 다르게, 렌더링 중 실행됨.
  // 배열에 있는 값은 useEffect의 두번째 인자와 마찬가지로 메모 함수 재실행의 기준값이다. 이게 변경되면 재계산됨.
  // 배열이 없는 경우 매 렌더링 때마다 새 값을 계산하게 될 것
  return <div>{z}</div>
}
```

### useCallback
- 메모이제이션 된 콜백 반환 (함수를 메모이제이션하기)
- 사용법
```javascript
const memoizedCallback = useCallback(
  () => {
    doSomething(a, b); //doSomething을
  },
  [a, b], //a,b가 변경될 때까지 저장해두고 재사용
);

const nonMemoizedCallback = () => {
    doSomething(a, b); 
} //이렇게 선언되어 있었을 경우, 렌더링 될 때마다 새로운 nonMemoizedCallback가 생성
```
- 그런 부분에서 성능 문제가 있을까?
    * js에서, 완전히 같은 로직이어도 다른 변수에 할당되어 있으면 함수도 객체로 취급하기 때문에 다른 함수로 인식함.
    * 즉, React 컴포넌트 함수 내에서 어떤 함수를 다른 함수의 인자로 넘기거나 자식 컴포넌트의 prop으로 넘길 때 예상치 못한 성능 문제 발생 가능
```javascript
// 문제가 생길 수 있는 컴포넌트
function Profile({ userId }) {
  const [user, setUser] = useState(null)

  const fetchUser = (userId) =>
    fetch(`https://your-api.com/users/${userId}`)
      .then((response) => response.json())
      .then(({ user }) => user)

  useEffect(() => {
    fetchUser(userId).then((user) => setUser(user))
  }, [fetchUser, userId]) //의존 배열에 들어가 있는 함수
}

// 콜백 메모이제이션으로 의도한 대로 실행하도록 한 컴포넌트
function Profile({ userId }) {
  const [user, setUser] = useState(null)

  const fetchUser = useCallback( // 콜백 메모이제이션 (컴포넌트가 리렌더링되어도 참조값 동일)
    (userId) =>
      fetch(`https://your-api.com/users/${userId}`)
        .then((response) => response.json())
        .then(({ user }) => user),
    []
  )

  useEffect(() => {
    fetchUser(userId).then((user) => setUser(user))
  }, [fetchUser, userId])

  // ...
}

// 또 다른 예시: 제어할 스테이트가 많은 경우
function Light({ room, on, toggle }) {
  console.log({ room, on })
  return (
    <button onClick={toggle}>
      {room} {on ? "💡" : "⬛"}
    </button>
  )
}
export default React.memo(Light)
// React.Memo: 컴포넌트에서 리렌더링이 필요한 상황에서만 리렌더링을 하도록 설정

function SmartHome() {
  const [masterOn, setMasterOn] = useState(false)
  const [kitchenOn, setKitchenOn] = useState(false)
  const [bathOn, setBathOn] = useState(false)

// 이렇게 할 경우, 콘솔에 세 스테이트 모두의 값이 찍힘.
  const toggleMaster = () => setMasterOn(!masterOn)
  const toggleKitchen = () => setKitchenOn(!kitchenOn)
  const toggleBath = () => setBathOn(!bathOn)

// 콜백 메모이제이션으로 변화가 없는 자식 컴포넌트의 불필요한 리렌더링을 막음
  const toggleMaster = useCallback(() => setMasterOn(!masterOn), [masterOn])
  const toggleKitchen = useCallback(() => setKitchenOn(!kitchenOn), [kitchenOn])
  const toggleBath = useCallback(() => setBathOn(!bathOn), [bathOn])

  return (
    <>
      <Light room="침실" on={masterOn} toggle={toggleMaster} />
      <Light room="주방" on={kitchenOn} toggle={toggleKitchen} />
      <Light room="욕실" on={bathOn} toggle={toggleBath} />
    </>
  )
}
```

### useLayoutEffect
- 공식 문서에서는 일단 useEffect를 사용할 것을 권함.
- useEffect와 동일 + 모든 DOM 변경 후에 동기적으로 발생. 

### useDebugValue
- React 개발자도구에서 custom Hook 레이블을 표시
- 사용법
```javascript
// 사용자가 만든 커스텀 훅 안에 아래와 같은 라인을 포함한다면,
useDebugValue(isOnline ? 'Online' : 'Offline');
// 개발자 도구에서 "FriendStatus: Online" 와 같이 표시됨.
```

## Custom Hook
```javascript
// 커스텀 훅, useFriendStatus
import { useState, useEffect } from 'react';

function useFriendStatus(friendID) {
  const [isOnline, setIsOnline] = useState(null);

  useEffect(() => {
    function handleStatusChange(status) {
      setIsOnline(status.isOnline);
    }

    ChatAPI.subscribeToFriendStatus(friendID, handleStatusChange);
    return () => {
      ChatAPI.unsubscribeFromFriendStatus(friendID, handleStatusChange);
    };
  });

  return isOnline;
}

// 커스텀 훅을 사용하는 컴포넌트
function FriendListItem(props) {
  const isOnline = useFriendStatus(props.friend.id);

  return (
    <li style={{ color: isOnline ? 'green' : 'black' }}>
      {props.friend.name}
    </li>
  );
}
```
- 컴포넌트 로직을 함수로 뽑아내어 재사용할 수 있음
- [추천하는 글](https://engineering.linecorp.com/ko/blog/line-securities-frontend-3/)
- 특징
    * 함수 이름을 use로 시작하도록 짓기
    * 다른 hook을 호출할 수 있음 
    * 조건부 함수가 아니어야 함
    * 파라미터/리턴값을 사용자가 정의할 수 있음
    * 같은 커스텀 훅을 다른 곳에서 호출하면, 두 컴포넌트는 스테이트를 공유하지 않음

## React Hook을 다룰 때 알고 있으면 좋은 것들

### 렌더링마다 고유한 Props, State, 이벤트 핸들러, 이펙트가 있다.
1. 컴포넌트 안에 있는 모든 함수는 렌더링 될 당시의 상수화된 변수값들을 사용한다.
    - 컴포넌트 내부의 모든 로컬 변수는 상수
    - 컴포넌트 내부의 모든 변수나 함수는 렌더링 단위로 `갇힌다`
2. 이펙트는 콜백함수다. 콜백함수는 자기가 생성될 당시의 state를 `바라본다` 
    - useEffect에 넘겨준 익명함수는 컴포넌트 내부에 있는 state와 props를 렌더링 단위로 기억한다.
    - 왜? `클로저`의 특성 때문

### 클로저
- 의미? 자바스크립트에서 함수가 자신이 생성될 때 당시의 상황(주변 변수)를 기억하는 것. 이때, 값은 상수.
- 스코프 체인? 자신에게 없는 변수를 참조하라고 할 때, 주변 변수까지 찾아보는 것
- 예)
    * 클래스형에서 `this.state.something`로 객체 내에 저장된 변수의 값을 가져온다. (최신 상태 보장)
    * 함수형에서는 `something`으로 이벤트 발생 당시의 값을 가져온다.

### cleanup
- 의미? 필요한 경우, 이펙트를 렌더링마다 실행하면 메모리 누수의 가능성이 있을 때 사용하는 함수
- 목적? 구독과 같은 이펙트를 `되돌리는` 것
- 특징? 컴포넌트가 랜더링 안에 있는 모든 함수는 (이벤트 핸들러, 이펙트, 타임아웃이나 그 안에서 호출되는 API 등) 랜더가 호출될 때 정의된 props와 state 값을 잡아둔다. 이펙트의 클린업은 그래서 최신 props를 읽지 않는다. 클린업이 정의된 시점의 랜더링에 있던 값을 읽는다.

## 참고자료
- [리액트 공식 문서](https://ko.reactjs.org/docs/hooks-intro.html)
- [코드스쿼드 - 참고 코드 (todo)](https://www.youtube.com/watch?v=y52Av3JxNW4&ab_channel=%EC%BD%94%EB%93%9C%EC%8A%A4%EC%BF%BC%EB%93%9C)
- [벨로퍼트와 함께하는 모던 리액트](https://react.vlpt.us/)
- [Engineering Blog by Dale Seo](https://www.daleseo.com/?tag=React)
- [오픈소스 컨설팅 - React에서 useRef 사용하기](https://tech.osci.kr/2019/10/10/82068584/)
- [Kelly Woo - useImperativeHandle](https://kelly-kh-woo.medium.com/react-hook-useimperativehandle-89fee716d80)
- [useEffect 완벽 가이드](https://overreacted.io/a-complete-guide-to-useeffect/)
- [useEffect 완벽 가이드(한글 번역)](https://overreacted.io/ko/a-complete-guide-to-useeffect/)