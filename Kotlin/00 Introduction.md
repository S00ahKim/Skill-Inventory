# Introduction
## Kotlin이란?
- 정의: Statically typed programming language for modern multiplatform applications
- 정적 타입? 컴파일 시점에 객체의 타입이 결정되기 때문에 실행 시점에 문제가 발생할 확률이 동적 타입 언어보다 적음

## Why Kotlin?
1. 문법의 간결함
    - 직관적, 간결함, 적은 중복
    - 익숙함: scala, C#, groovy, swift, ...
2. Null Safety
    - 자바: 런타임에 NPE
    - 코틀린: 컴파일 시점에 null 관련 문제를 처리하게 함
        * 변수나 파라미터에 null을 할당하려면 해당 변수나 파라미터가 null일 수 있다는 것을 명시적으로 표현
        * ex. `null일수도있는변수명?`
3. 기존의 자바 코드와 호환 가능
    - Java <-> Kotlin 쌍방 호출 가능
        * 코틀린 역시 JVM 기반
        * 자바 코드 -(`javac`)-> 컴파일된 바이트 코드 (class) -(`JVM`)-> 기계어
        * 코틀린 코드 -(`kotlinc`)-> 컴파일된 바이트 코드 (class) -(`JVM`)-> 기계어
    - 기존 자바 라이브러리 활용 가능
    - (완벽하지는 않으나) 자바 코드로 변환 가능
    - etc: 자바스크립트 컴파일 지원
4. 강력한 기능
    - 확장 함수
    - 연산자 오버로딩: 클래스에 사용할 수 없는 기본 연산자(+, -, *, /, ++, -- 등)를 클래스에 사용할 수 있게 연산자를 정의
    - 늦은(lazy) 필드 초기화 (객체 필드를 항상 초기화하고, 바로 사용하지 않음)
    - DSL (Domain-Specific Language)
    - 코루틴 (비동기, Actor)
    - Delegate
    - Reified
    - Smart Cast (`is`, `as`)
    - 타입 추론
5. 툴 친화적
    - 개발사: JetBrains
    - 지원 IDE: IntelliJ, Eclipse, ...
6. 안드로이드만...?
    - 구글이 안드로이드 공식 언어로 채택
    - 카카오톡 채팅 서버: 코드량 감소, 생산성 향상


## 생각해 보아야 할 단점들
1. namespace가 없다. (자바의 package 개념. 가져다 쓸 때 헷갈릴 수 있음.)
2. static modifier가 없다. (모든 것을 `companion object`로 처리해야 함)
3. 스칼라보다는 훨씬 낫지만, 빌드 시간이 자바에 비해 조금 더 걸린다.
4. 디폴트 설정이 `final` (즉, 기본으로 상속 불가. 안정적이고, 좀 더 빠르기 때문이라 그럴 듯)
5. 바이트코드 사이즈가 자바에 비해 늘어난다.
6. 커뮤니티 부족


## IntelliJ에서 코틀린 사용하기
1. [새로운 프로젝트 (코틀린, 인텔리제이 커뮤니티 코틀린 프로젝트 생성(개발환경 설정))](https://sas-study.tistory.com/322)
2. [기존 프로젝트 (Mix Java and Kotlin in one project)](https://www.jetbrains.com/help/idea/mixing-java-and-kotlin-in-one-project.html)


## 코틀린에서 자바 코드 읽기
1. 코틀린 코드에서 자바 메서드 호출하기
```kotlin
// SampleKotlin.kt
fun main(args: Array<String>){
   val rectangleArea: Int = SampleJavaClass.rectangleArea(10, 20)  
   println("코틀린 코드 안에서 자바 클래스 사용하기" + rectangleArea)
  
}
```
```java
// SampleJavaClass.java
public class SampleJavaClass{
    public static void main(String[] args){  
        // ...
    }
    public static int rectangleArea(int x, int y){  
        int result = x * y;  
        return result;  
    }  
}
```

2. 다른 패키지에 있는 코틀린 코드에서 자바 코드 호출하기
```kotlin
// SampleKotlin.kt
package mykotlinpackage 
import mypackage.SampleJava //자바 클래스 import

fun main(args: Array<String>){ 
    val result= SampleJava.display() 
    println(result) 
}
```
```java
// SampleJava.java
package mypackage;
    public class SampleJava{
        public static void main(String[] args){  
            // ...
        }
        public static void display(){ 
            System.out.println("코드를 불러주세요"); 
        } 
     }
```

## 자바 코드를 코틀린 코드로 변환하기
ex. `HomeLayoutAdmin.java` -> `HomeLayoutAdmin.kt`
1. 원본 자바 코드
```java
// HomeLayoutAdmin.java
package com.company.home;
import java.util.Date;
import lombok.Data;

@Data
public class HomeLayoutAdmin {
	private int layoutNumber;
	private String layoutTitle;
	private String registerID;
	private Date registerDatetime;
	private String lastModifyChargeID;
	private Date lastModifyDatetime;
}
```
2. lombok 해제 (refactor > delombok)
```java
package com.company.home;
import java.util.Date;

public class HomeLayoutAdmin {
	private int layoutNumber;
	private String layoutTitle;
	private String registerID;
	private Date registerDatetime;
	private String lastModifyChargeID;
	private Date lastModifyDatetime;

	public HomeLayoutAdmin() {
	}

	public int getLayoutNumber() {
		return this.layoutNumber;
	}

	public String getLayoutTitle() {
		return this.layoutTitle;
	}

	public String getRegisterID() {
		return this.registerID;
	}

	public Date getRegisterDatetime() {
		return this.registerDatetime;
	}

	public String getLastModifyChargeID() {
		return this.lastModifyChargeID;
	}

	public Date getLastModifyDatetime() {
		return this.lastModifyDatetime;
	}

	public void setLayoutNumber(int layoutNumber) {
		this.layoutNumber = layoutNumber;
	}

	public void setLayoutTitle(String layoutTitle) {
		this.layoutTitle = layoutTitle;
	}

	public void setRegisterID(String registerID) {
		this.registerID = registerID;
	}

	public void setRegisterDatetime(Date registerDatetime) {
		this.registerDatetime = registerDatetime;
	}

	public void setLastModifyChargeID(String lastModifyChargeID) {
		this.lastModifyChargeID = lastModifyChargeID;
	}

	public void setLastModifyDatetime(Date lastModifyDatetime) {
		this.lastModifyDatetime = lastModifyDatetime;
	}

	public boolean equals(final Object o) {
		if (o == this) return true;
		if (!(o instanceof HomeLayoutAdmin)) return false;
		final HomeLayoutAdmin other = (HomeLayoutAdmin) o;
		if (!other.canEqual((Object) this)) return false;
		if (this.getLayoutNumber() != other.getLayoutNumber()) return false;
		final Object this$layoutTitle = this.getLayoutTitle();
		final Object other$layoutTitle = other.getLayoutTitle();
		if (this$layoutTitle == null ? other$layoutTitle != null : !this$layoutTitle.equals(other$layoutTitle))
			return false;
		final Object this$registerID = this.getRegisterID();
		final Object other$registerID = other.getRegisterID();
		if (this$registerID == null ? other$registerID != null : !this$registerID.equals(other$registerID))
			return false;
		final Object this$registerDatetime = this.getRegisterDatetime();
		final Object other$registerDatetime = other.getRegisterDatetime();
		if (this$registerDatetime == null ? other$registerDatetime != null : !this$registerDatetime.equals(other$registerDatetime))
			return false;
		final Object this$lastModifyChargeID = this.getLastModifyChargeID();
		final Object other$lastModifyChargeID = other.getLastModifyChargeID();
		if (this$lastModifyChargeID == null ? other$lastModifyChargeID != null : !this$lastModifyChargeID.equals(other$lastModifyChargeID))
			return false;
		final Object this$lastModifyDatetime = this.getLastModifyDatetime();
		final Object other$lastModifyDatetime = other.getLastModifyDatetime();
		if (this$lastModifyDatetime == null ? other$lastModifyDatetime != null : !this$lastModifyDatetime.equals(other$lastModifyDatetime))
			return false;
		return true;
	}

	protected boolean canEqual(final Object other) {
		return other instanceof HomeLayoutAdmin;
	}

	public int hashCode() {
		final int PRIME = 59;
		int result = 1;
		result = result * PRIME + this.getLayoutNumber();
		final Object $layoutTitle = this.getLayoutTitle();
		result = result * PRIME + ($layoutTitle == null ? 43 : $layoutTitle.hashCode());
		final Object $registerID = this.getRegisterID();
		result = result * PRIME + ($registerID == null ? 43 : $registerID.hashCode());
		final Object $registerDatetime = this.getRegisterDatetime();
		result = result * PRIME + ($registerDatetime == null ? 43 : $registerDatetime.hashCode());
		final Object $lastModifyChargeID = this.getLastModifyChargeID();
		result = result * PRIME + ($lastModifyChargeID == null ? 43 : $lastModifyChargeID.hashCode());
		final Object $lastModifyDatetime = this.getLastModifyDatetime();
		result = result * PRIME + ($lastModifyDatetime == null ? 43 : $lastModifyDatetime.hashCode());
		return result;
	}

	public String toString() {
		return "HomeLayoutAdmin(layoutNumber=" + this.getLayoutNumber() + ", layoutTitle=" + this.getLayoutTitle() + ", registerID=" + this.getRegisterID() + ", registerDatetime=" + this.getRegisterDatetime() + ", lastModifyChargeID=" + this.getLastModifyChargeID() + ", lastModifyDatetime=" + this.getLastModifyDatetime() + ")";
	}
}
```
3. 코틀린 디펜던시 추가 (Tool > Kotlin > Configure Kotlin in Project)
4. 코틀린 코드로 변환하기 (Convert Java File to Kotlin File)
```kotlin
// HomeLayoutAdmin.kt
package com.company.home
import com.company.home.HomeLayoutAdmin
import java.util.*

class HomeLayoutAdmin {
    var layoutNumber = 0
    var layoutTitle: String? = null
    var registerID: String? = null
    var registerDatetime: Date? = null
    var lastModifyChargeID: String? = null
    var lastModifyDatetime: Date? = null
    override fun equals(o: Any?): Boolean {
        if (o === this) return true
        if (o !is HomeLayoutAdmin) return false
        val other = o
        if (!other.canEqual(this as Any)) return false
        if (layoutNumber != other.layoutNumber) return false
        val `this$layoutTitle`: Any? = layoutTitle
        val `other$layoutTitle`: Any? = other.layoutTitle
        if (if (`this$layoutTitle` == null) `other$layoutTitle` != null else `this$layoutTitle` != `other$layoutTitle`) return false
        val `this$registerID`: Any? = registerID
        val `other$registerID`: Any? = other.registerID
        if (if (`this$registerID` == null) `other$registerID` != null else `this$registerID` != `other$registerID`) return false
        val `this$registerDatetime`: Any? = registerDatetime
        val `other$registerDatetime`: Any? = other.registerDatetime
        if (if (`this$registerDatetime` == null) `other$registerDatetime` != null else `this$registerDatetime` != `other$registerDatetime`) return false
        val `this$lastModifyChargeID`: Any? = lastModifyChargeID
        val `other$lastModifyChargeID`: Any? = other.lastModifyChargeID
        if (if (`this$lastModifyChargeID` == null) `other$lastModifyChargeID` != null else `this$lastModifyChargeID` != `other$lastModifyChargeID`) return false
        val `this$lastModifyDatetime`: Any? = lastModifyDatetime
        val `other$lastModifyDatetime`: Any? = other.lastModifyDatetime
        return if (if (`this$lastModifyDatetime` == null) `other$lastModifyDatetime` != null else `this$lastModifyDatetime` != `other$lastModifyDatetime`) false else true
    }

    protected fun canEqual(other: Any?): Boolean {
        return other is HomeLayoutAdmin
    }

    override fun hashCode(): Int {
        val PRIME = 59
        var result = 1
        result = result * PRIME + layoutNumber
        val `$layoutTitle`: Any? = layoutTitle
        result = result * PRIME + (`$layoutTitle`?.hashCode() ?: 43)
        val `$registerID`: Any? = registerID
        result = result * PRIME + (`$registerID`?.hashCode() ?: 43)
        val `$registerDatetime`: Any? = registerDatetime
        result = result * PRIME + (`$registerDatetime`?.hashCode() ?: 43)
        val `$lastModifyChargeID`: Any? = lastModifyChargeID
        result = result * PRIME + (`$lastModifyChargeID`?.hashCode() ?: 43)
        val `$lastModifyDatetime`: Any? = lastModifyDatetime
        result = result * PRIME + (`$lastModifyDatetime`?.hashCode() ?: 43)
        return result
    }

    override fun toString(): String {
        return "HomeLayoutAdmin(layoutNumber=" + layoutNumber + ", layoutTitle=" + layoutTitle + ", registerID=" + registerID + ", registerDatetime=" + registerDatetime + ", lastModifyChargeID=" + lastModifyChargeID + ", lastModifyDatetime=" + lastModifyDatetime + ")"
    }
}
```
5. 이후 적절하게 수정 필요


## Reference
- [(이 디렉토리가 다루는 문서) Kotilin Docs](https://kotlinlang.org/docs/reference/basic-syntax.html)
- [(코드 스니펫 작성 및 실행) Kotlin Playground](https://play.kotlinlang.org/)
- [자바 개발자 관점의 - 왜 코틀린인가? by 강현식](https://www.youtube.com/watch?v=HhifPEExguA&ab_channel=JetBrainsTV)
- [Google I/O 2017 참관기 - Kotlin](https://d2.naver.com/helloworld/7543578)
- [노마드 코더 - 코틀린이 자바를 대체할 수 있을까?](https://www.youtube.com/watch?v=8gseVzeMOzk&ab_channel=%EB%85%B8%EB%A7%88%EB%93%9C%EC%BD%94%EB%8D%94NomadCoders)
- [Calling Java Codes from Kotlin](https://medium.com/kayvan-kaseb/calling-java-codes-from-kotlin-b74890fb4a78)
- [Java를 Kotlin으로 변환하기(Convert Java to Kotlin)](https://thatisgood.tistory.com/entry/Java-to-Kotlin)
- [Kotlin 도입 과정에서 만난 문제와 해결 방법](https://d2.naver.com/helloworld/6685007)
- [스프링캠프 2018 [TrackB Session2] : 쿠팡 Kotlin Backend 적용기](https://www.youtube.com/watch?v=bhI1hMOcT-4)