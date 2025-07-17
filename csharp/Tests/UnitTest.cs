using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(true, "leetcode", new[] { "leet", "code" })]
    public void TestMethod1(bool exp, string s, string[] d)
    {
        Assert.AreEqual(exp, sol.WordBreak(s, d));
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