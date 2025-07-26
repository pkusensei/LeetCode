using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { "(())()", "()()()" }, "()())()")]
    [DataRow(new[] { "(a())()", "(a)()()" }, "(a)())()")]
    [DataRow(new[] { "" }, ")(")]
    public void TestMethod1(string[] exp, string s)
    {
        Assert.IsTrue(exp.Order().SequenceEqual(sol.RemoveInvalidParentheses(s).Order()));
    }

    [TestMethod]
    public void TestMethod2()
    {
    }

    [TestMethod]
    public void TestMethod3()
    {
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}