using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { "bab", "dab", "cab" }, new[] { 1, 2, 2 }, new[] { "bab", "dab" })]
    public void TestMethod1(string[] w, int[] g, string[] e)
    {
        Assert.AreEqual(e.Print(), sol.GetWordsInLongestSubsequence(w, g).Print());
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