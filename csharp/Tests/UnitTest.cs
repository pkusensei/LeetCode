using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(19, "cdbcbbaaabab", 4, 5)]
    [DataRow(20, "aabbaaxybbaabb", 5, 4)]
    public void TestMethod1(int exp, string s, int x, int y)
    {
        Assert.AreEqual(exp, sol.MaximumGain(s, x, y));
        Assert.AreEqual(exp, sol.WithCounting(s, x, y));
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