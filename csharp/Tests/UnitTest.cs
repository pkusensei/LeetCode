using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow("12233", 4, -1)]
    [DataRow("1122211", 3, 1)]
    [DataRow("110", 3, -1)]
    public void TestMethod1(string s, int k, int exp)
    {
        Assert.AreEqual(exp, sol.MaxDifference(s, k));
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