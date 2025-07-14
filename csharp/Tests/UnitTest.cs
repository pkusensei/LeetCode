using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow("aabcc", "dbbca", "aadbbcbcac", true)]
    public void TestMethod1(string a, string b, string c, bool exp)
    {
        Assert.AreEqual(exp, sol.IsInterleave(a, b, c));
        Assert.AreEqual(exp, sol.With1dDp(a, b, c));
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