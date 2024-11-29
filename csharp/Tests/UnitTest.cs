using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(false, "&(|(f))")]
    [DataRow(true, "|(f,f,f,t)")]
    [DataRow(true, "!(&(f,t))")]
    public void TestMethod1(bool exp, string s)
    {
        Assert.AreEqual(exp, sol.ParseBoolExpr(s));
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