using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 1, -3, 4 }, 1, 6, 2)]
    [DataRow(new[] { 3, -4, 5, 1, -2 }, -4, 5, 4)]
    [DataRow(new[] { 4, -7, 2 }, 3, 6, 0)]
    public void TestMethod1(int[] nums, int lower, int upper, int exp)
    {
        Assert.AreEqual(exp, sol.NumberOfArrays(nums, lower, upper));
    }

    [TestMethod]
    public void TestMethod2()
    { }

    [TestMethod]
    public void TestMethod3()
    {
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}