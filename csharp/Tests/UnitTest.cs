using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(2, "1 + 1")]
    [DataRow(3, " 2-1 + 2 ")]
    [DataRow(23, "(1+(4+5+2)-3)+(6+8)")]
    public void TestMethod1(int exp, string s)
    {
        Assert.AreEqual(exp, sol.Calculate(s));
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