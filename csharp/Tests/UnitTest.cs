using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow("aaacecaaa", "aacecaaa")]
    [DataRow("dcbabcd", "abcd")]
    public void TestMethod1(string exp, string s)
    {
        Assert.AreEqual(exp, sol.ShortestPalindrome(s));
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