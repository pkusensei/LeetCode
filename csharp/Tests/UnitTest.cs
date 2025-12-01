using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(3, "abcde", new[] { "a", "bb", "acd", "ace" })]
    public void TestMethod1(int exp, string s, string[] w)
    {
        Assert.AreEqual(exp, sol.NumMatchingSubseq(s, w));
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