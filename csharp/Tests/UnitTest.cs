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
        var a = TreeNode.Make([2, 3, 1, 3, 1, null, 1]);
        Assert.AreEqual(2, sol.PseudoPalindromicPaths(a));
    }

    [TestMethod]
    public void TestMethod2()
    {
        var a = TreeNode.Make([2, 1, 1, 1, 3, null, null, null, null, null, 1]);
        Assert.AreEqual(1, sol.PseudoPalindromicPaths(a));
    }

    [TestMethod]
    public void TestMethod3()
    {
        var a = TreeNode.Make([9]);
        Assert.AreEqual(1, sol.PseudoPalindromicPaths(a));
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}