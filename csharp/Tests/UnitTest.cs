using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new int[] { 8, 5, 1, 7, 10, 12 }, "[8,5,10,1,7,null,12]")]
    [DataRow(new int[] { 1, 3 }, "[1,null,3]")]
    [DataRow(new int[] { 4, 2 }, "[4,2]")]
    public void TestMethod1(int[] nums, string c)
    {
        var b = sol.BstFromPreorder(nums);
        Assert.AreEqual(c, b.ToString());
    }

    [TestMethod]
    public void TestMethod2()
    {
    }

    [TestMethod]
    public void TestMethod3()
    {
    }
}