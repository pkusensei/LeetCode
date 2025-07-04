using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow("aa", "a", false)]
    [DataRow("aa", "a*", true)]
    [DataRow("ab", ".*", true)]
    public void TestMethod1(string s, string p, bool exp)
    {
        Assert.AreEqual(exp, sol.IsMatch(s, p));
        Assert.AreEqual(exp, sol.WithDp(s, p));
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