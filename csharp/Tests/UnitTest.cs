using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 4, 5, 6, 7, 0, 1, 2 }, 0, 4)]
    public void TestMethod1(int[] nums, int target, int exp)
    {
        Assert.AreEqual(exp, sol.Search(nums, target));
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