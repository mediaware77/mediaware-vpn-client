import clsx from 'clsx';
import './style.scss';
import { useQuery } from '@tanstack/react-query';
import { getVersion } from '@tauri-apps/api/app';
import { isPresent } from '../../utils/isPresent';
import logoSrc from './assets/logo.png';
import { ConnectionWatcher } from './components/ConnectionWatcher/ConnectionsWatcher';

interface Props {
  variant: 'compact' | 'desktop';
}

export const WindowHeader = ({ variant }: Props) => {
  const { data: appVersion } = useQuery({
    queryFn: getVersion,
    queryKey: ['app-version'],
  });

  const version = () => {
    if (appVersion) {
      return `Version ${appVersion}`;
    }
  };

  return (
    <div id="window-header" className={clsx(`variant-${variant}`)}>
      <LogoIcon size={variant === 'desktop' ? 33 : 48} />
      <div className="info">
        <p className="label">Mediaware VPN</p>
        {variant === 'compact' && <ConnectionWatcher placement="bottom-start" />}
        {variant === 'desktop' && isPresent(appVersion) && (
          <p className="version">{version()}</p>
        )}
      </div>
      {variant === 'desktop' && (
        <div className="right">
          <ConnectionWatcher placement={'bottom-end'} />
        </div>
      )}
    </div>
  );
};

const LogoIcon = ({ size = 48 }: { size?: number }) => (
  <img src={logoSrc} width={size} height={size} alt="Mediaware VPN" />
);
