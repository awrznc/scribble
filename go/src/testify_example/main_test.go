package main

import (
	"fmt"
	"testing"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/suite"
	"github.com/stretchr/testify/mock"
)

type PiyoSuite struct {
	suite.Suite
	Value int
}

func (suite *PiyoSuite) SetupTest() {
	fmt.Println("SetupTest")
}

func (suite *PiyoSuite) BeforeTest(suiteName, testName string) {
	fmt.Println("BeforeTest", suiteName, testName)
}

func (suite *PiyoSuite) TestExec1() {
	fmt.Println("TestExec1")
	suite.Equal(Piyo(), 2)
}

func (suite *PiyoSuite) AfterTest(suiteName, testName string) {
	fmt.Println("AfterTest", suiteName, testName)
}

// === mock begin ===
type PiyoMock struct {
	mock.Mock
}

func (mock *PiyoMock) Foo() int {
	// Fooに対応したMockが呼び出される
	return mock.Called().Int(0)
}
// === mock end ===

// Suite Run
func TestPiyoSuite(tester *testing.T) {
	suite.Run(tester, new(PiyoSuite))

	// Mock
	// OnでセットしてCallで呼び出す
	piyoMockObject := new(PiyoMock)
	piyoMockObject.On("Foo").Return(10)

	// 元の実装の場合は1が返るが、今回はMockを利用しているため10が返る
	fmt.Println("Bar(piyoMockObject):", Bar(piyoMockObject))
}
// =>
// SetupTest
// BeforeTest PiyoSuite TestExec1
// TestExec1
// AfterTest PiyoSuite TestExec1
// Bar(piyoMockObject): 10
// --- FAIL: TestPiyoSuite (0.00s)
//     --- FAIL: TestPiyoSuite/TestExec1 (0.00s)
//         main_test.go:26:
//                 Error Trace:    main_test.go:26
//                 Error:          Not equal:
//                                 expected: 0
//                                 actual  : 2
//                 Test:           TestPiyoSuite/TestExec1


// Assert Run
func TestPiyo(tester *testing.T) {
	assert.Equal(tester, Piyo(), 1, "Piyo() not equal zero.")
}
// =>
// --- FAIL: TestPiyo (0.00s)
//     main_test.go:74:
//                 Error Trace:    main_test.go:74
//                 Error:          Not equal:
//                                 expected: 0
//                                 actual  : 1
//                 Test:           TestPiyo
//                 Messages:       Piyo() not equal zero.
// FAIL
// exit status 1
// FAIL    testify_example 0.012s
