using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { "catsdogcats", "dogcatsdog", "ratcatdogcat" },
    new[] { "cat", "cats", "catsdogcats", "dog", "dogcatsdog", "hippopotamuses", "rat", "ratcatdogcat" })]
    [DataRow(new[] { "catdog" }, new[] { "cat", "dog", "catdog" })]
    public void TestMethod1(string[] exp, string[] w)
    {
        Assert.IsTrue(exp.Order().SequenceEqual(sol.FindAllConcatenatedWordsInADict(w).Order()));
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