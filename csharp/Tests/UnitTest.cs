using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(1, "a")]
    [DataRow(2, "cac")]
    [DataRow(6, "zab")]
    public void TestMethod1(int exp, string s)
    {
        Assert.AreEqual(exp, sol.FindSubstringInWraproundString(s));
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