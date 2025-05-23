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
        TreeNode a = TreeNode.Make([5, 3, 6, 5, 2, 5, 7, 1, 8, null, null, 6, 8]);
        Assert.AreEqual(3, sol.KthLargestPerfectSubtree(a, 2));
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