using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(1, new[] { 4, 2, 6, 1, 3 })]
    public void TestMethod1(int exp, int[] n)
    {
        var a = TreeNode.MakeInt(n);
        Assert.AreEqual(exp, sol.GetMinimumDifference(a));
        Assert.AreEqual(exp, sol.Inorder(a));
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