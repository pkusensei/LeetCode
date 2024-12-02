using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new int[] { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11 }, 11, 3, true)]
    [DataRow(new int[] { 1, 2, 3,}, 3,1,false)]
    public void TestMethod1(int[] nums, int n, int x, bool exp)
    {
        var a = TreeNode.Make(nums);
        Assert.AreEqual(exp, sol.BtreeGameWinningMove(a, n, x));
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