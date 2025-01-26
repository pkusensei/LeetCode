using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    public void TestMethod1()
    {
    }

    [TestMethod]
    [DataRow(new[] { "acca", "bbbb", "caca" }, "aba", 6)]
    [DataRow(new[] { "abba", "baab" }, "bab", 4)]
    public void TestMethod2(string[] words, string target, int exp)
    {
        Assert.AreEqual(exp, sol.NumWays(words, target));
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