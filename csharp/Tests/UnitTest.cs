using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow("abac", "cab", "cabac")]
    [DataRow("aaaaaaaa", "aaaaaaaa", "aaaaaaaa")]
    public void TestMethod1(string s1, string s2, string exp)
    {
        Assert.AreEqual(exp, sol.ShortestCommonSupersequence(s1, s2));
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