using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow("great", "rgeat", true)]
    [DataRow("abcde", "caebd", false)]
    [DataRow("abb", "bba", true)]
    public void TestMethod1(string s1, string s2, bool exp)
    {
        Assert.AreEqual(exp, sol.IsScramble(s1, s2));
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