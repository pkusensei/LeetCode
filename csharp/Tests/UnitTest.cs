using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new int[] { 2, 3, 5, 8, 13, 21, 34 }, "[2,5,3,8,13,21,34]")]
    [DataRow(new int[] { 7, 13, 11 }, "[7,11,13]")]
    public void TestMethod1(int[] nums, string exp)
    {
        var a = TreeNode.Make(nums);
        Assert.AreEqual(exp, sol.ReverseOddLevels(a).ToString());
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