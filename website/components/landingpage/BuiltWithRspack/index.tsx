import amazonLogo from './assets/amazon.svg';
import bitDevLogo from './assets/bit.svg';
import bytedanceLogo from './assets/bytedance.svg';
import intuitLogo from './assets/intuit.svg';
import microsoftLogo from './assets/microsoft.svg';
import styles from './index.module.scss';

const BuiltWithRsPack: React.FC = () => {
  return (
    <div className={styles.container}>
      <div className={styles.innerContainer}>
        <h2 className={styles.title}>Built with Rspack</h2>
        <div className={styles.logos}>
          <div className={styles.logo}>
            <img src={bitDevLogo} alt="bit.dev" />
            <span className={styles.bitDevText}>bit.dev</span>
          </div>
          <div className={styles.logo}>
            <img src={microsoftLogo} alt="Microsoft" />
          </div>
          <div className={styles.logo}>
            <img src={amazonLogo} alt="Amazon" />
          </div>
          <div className={styles.logo}>
            <img src={bytedanceLogo} alt="ByteDance" />
          </div>
          <div className={styles.logo}>
            <img src={intuitLogo} alt="Intuit" />
          </div>
        </div>
      </div>
    </div>
  );
};

export default BuiltWithRsPack;
