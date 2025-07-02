using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow("aabbccdd", 7, 5)]
    [DataRow("aabbccdd", 8, 1)]
    [DataRow("aaabbb", 3, 8)]
    public void TestMethod1(string s, int k, int exp)
    {
        Assert.AreEqual(exp, sol.PossibleStringCount(s, k));
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