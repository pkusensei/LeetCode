using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow("eleetminicoworoep", 13)]
    [DataRow("leetcodeisgreat", 5)]
    [DataRow("bcbcbc", 6)]
    public void TestMethod1(string s, int exp)
    {
        Assert.AreEqual(exp, sol.FindTheLongestSubstring(s));
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