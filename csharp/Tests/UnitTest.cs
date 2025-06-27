using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow("letsleetcode", 2, "let")]
    [DataRow("bb", 2, "b")]
    [DataRow("ab", 2, "")]
    public void TestMethod1(string s, int k, string exp)
    {
        Assert.AreEqual(exp, sol.LongestSubsequenceRepeatedK(s, k));
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