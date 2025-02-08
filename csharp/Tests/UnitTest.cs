using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 0, 1, 1, 3 }, 2, "[0,3,2,3]")]
    [DataRow(new[] { 2, 3, 4, 7 }, 3, "[5,2,6,5]")]
    [DataRow(new[] { 0, 1, 2, 2, 5, 7 }, 3, "[4,3,6,4,6,7]")]
    public void TestMethod1(int[] nums, int bit, string exp)
    {
        Assert.AreEqual(exp, sol.GetMaximumXor(nums, bit).Print());
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