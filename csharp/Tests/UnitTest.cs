using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { "/a", "/c/d", "/c/f" }, new[] { "/a", "/a/b", "/c/d", "/c/d/e", "/c/f" })]
    public void TestMethod1(string[] exp, string[] f)
    {
        Assert.IsTrue(exp.SequenceEqual(sol.RemoveSubfolders(f)));
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